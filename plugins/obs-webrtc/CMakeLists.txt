project(obs-webrtc)

include(FetchContent)

FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
  GIT_TAG v0.3.1)

FetchContent_MakeAvailable(Corrosion)

set(obs-webrtc_HEADERS webrtc-output.h)
set(obs-webrtc_SOURCES webrtc-service.c webrtc-output.c webrtc.c)

if(WIN32)
  set(MODULE_DESCRIPTION "OBS webrtc module")
  configure_file(${CMAKE_SOURCE_DIR}/cmake/bundle/windows/obs-module.rc.in
                 obs-webrtc.rc)
  list(APPEND obs-webrtc_SOURCES obs-webrtc.rc)
endif()

add_library(obs-webrtc MODULE ${obs-webrtc_SOURCES} ${obs-webrtc_HEADERS})

corrosion_import_crate(MANIFEST_PATH webrtc/Cargo.toml)
corrosion_set_env_vars(webrtc-rs-lib
                       BINDINGS_OUTPUT_DIR=${CMAKE_CURRENT_BINARY_DIR})

target_link_libraries(obs-webrtc OBS::libobs webrtc-rs-lib)
target_include_directories(obs-webrtc PRIVATE ${CMAKE_CURRENT_BINARY_DIR})

if(OS_MACOS)
  find_library(COREFOUNDATION CoreFoundation)
  find_library(SECURITY_FRAMEWORK Security)
  mark_as_advanced(COREFOUNDATION SECURITY_FRAMEWORK)
  target_link_libraries(obs-webrtc ${COREFOUNDATION} ${SECURITY_FRAMEWORK})
endif()

set_target_properties(obs-webrtc PROPERTIES FOLDER "plugins")

setup_plugin_target(obs-webrtc data)
