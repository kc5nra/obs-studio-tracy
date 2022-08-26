#include "obs-module.h"

struct webrtc_service_state {
	char *server, *key;
};

static const char *webrtc_service_name(void *unused)
{
	UNUSED_PARAMETER(unused);
	return obs_module_text("WebRTCService");
}

static void webrtc_service_update(void *data, obs_data_t *settings)
{
	struct webrtc_service_state *service = data;

	bfree(service->server);
	bfree(service->key);

	service->server = bstrdup(obs_data_get_string(settings, "server"));
	service->key = bstrdup(obs_data_get_string(settings, "key"));
}

static void webrtc_service_destroy(void *data)
{
	struct webrtc_service_state *service = data;

	bfree(service->server);
	bfree(service->key);
	bfree(service);
}

static void *webrtc_service_create(obs_data_t *settings, obs_service_t *service)
{
	struct webrtc_service_state *data =
		bzalloc(sizeof(struct webrtc_service_state));
	webrtc_service_update(data, settings);

	UNUSED_PARAMETER(service);
	return data;
}

static const char *webrtc_service_url(void *data)
{
	struct webrtc_service_state *service = data;
	return service->server;
}

static const char *webrtc_service_key(void *data)
{
	struct webrtc_service_state *service = data;
	return service->key;
}

static obs_properties_t *webrtc_service_properties(void *unused)
{
	UNUSED_PARAMETER(unused);

	obs_properties_t *ppts = obs_properties_create();

	obs_properties_add_text(ppts, "server", "URL", OBS_TEXT_DEFAULT);
	obs_properties_add_text(ppts, "key", obs_module_text("StreamKey"),
				OBS_TEXT_PASSWORD);
	return ppts;
}

static const char *webrtc_service_get_output_type(void *data)
{
	struct webrtc_service_state *service = data;
	return "webrtc_output";
}

static void webrtc_service_apply_encoder_settings(void *data,
						  obs_data_t *video_settings,
						  obs_data_t *audio_settings)
{
	UNUSED_PARAMETER(audio_settings);

	// For now, ensure maximum compatibility with webrtc peers
	if (video_settings) {
		obs_data_set_int(video_settings, "bf", 0);
		obs_data_set_int(video_settings, "keyint_sec", 2);
		obs_data_set_string(video_settings, "rate_control", "CBR");
		obs_data_set_string(video_settings, "x264opts",
				    "tune=zerolatency b-pyramid=0 scenecut=0");
		obs_data_set_string(video_settings, "profile", "baseline");
	}
}

struct obs_service_info webrtc_service_info = {
	.id = "webrtc",
	.get_name = webrtc_service_name,
	.create = webrtc_service_create,
	.destroy = webrtc_service_destroy,
	.update = webrtc_service_update,
	.get_properties = webrtc_service_properties,
	.get_url = webrtc_service_url,
	.get_key = webrtc_service_key,
	.get_output_type = webrtc_service_get_output_type,
	.apply_encoder_settings = webrtc_service_apply_encoder_settings,
};
