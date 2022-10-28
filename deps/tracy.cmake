project(tracy)


add_library(tracy_interface INTERFACE)
add_library(OBS::tracy_interface ALIAS tracy_interface)

target_compile_definitions(tracy_interface INTERFACE TRACY_MANUAL_LIFETIME TRACY_DELAYED_INIT)
target_include_directories(tracy_interface INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/tracy/public)

add_library(tracy STATIC)
add_library(OBS::tracy ALIAS tracy)

target_compile_definitions(tracy PRIVATE TRACY_EXPORTS)

target_sources(tracy PRIVATE tracy/public/TracyClient.cpp)

target_include_directories(tracy PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/tracy/public)
