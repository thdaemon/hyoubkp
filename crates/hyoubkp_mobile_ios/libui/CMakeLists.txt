cmake_minimum_required(VERSION 3.25)

project(hmui_ios)

enable_language(OBJC)

add_library(hmui_ios STATIC
	hmui.m
	main.m
	MainViewController.m
)

target_compile_options(hmui_ios PRIVATE -fobjc-arc)

install(TARGETS hmui_ios DESTINATION .)