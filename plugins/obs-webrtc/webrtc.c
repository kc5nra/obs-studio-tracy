#include <obs-module.h>
#include "bindings.h"

OBS_DECLARE_MODULE()
OBS_MODULE_USE_DEFAULT_LOCALE("obs-webrtc", "en-US")
MODULE_EXPORT const char *obs_module_description(void)
{
	return "OBS webrtc module";
}

extern struct obs_output_info webrtc_output_info;
extern struct obs_service_info webrtc_service_info;

bool obs_module_load(void)
{
	obs_webrtc_install_logger();

	obs_register_output(&webrtc_output_info);
	obs_register_service(&webrtc_service_info);

	return true;
}
