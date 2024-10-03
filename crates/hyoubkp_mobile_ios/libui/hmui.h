#pragma once

#include <stdint.h>
#include <stdbool.h>

int appui_main(int argc, char *argv[]);

char const *appui_string_cstr(void *s);
void appui_string_dealloc(void *s);

void appui_uikit_control_set_enabled(void *control, int b);
void appui_uikit_label_set_text(void *label, char const *s);
char const *appui_uikit_textField_get_text(void *textField);
void appui_uikit_textField_set_text(void *textField, char const *s);

void appui_uikit_alertctrl(void *vc, char const *title, char const *message, void *callback, void *callback_userdata);

void *appui_fs_document_path();

bool appui_userdefaults_fsync();
void appui_userdefaults_set_i32(char const *key, int32_t i);
int32_t appui_userdefaults_get_i32(char const *key);

#define UI_GET_PROPERTY(c, p) \
    void *appui_##c##_##p(void *obj);

#define UI_GET_PROPERTY_IMPL(c, p) \
    void *appui_##c##_##p(void *obj) { \
        c *o = (__bridge c *)obj; \
        return (__bridge void *)o.p; \
    }

#define APP_ACTION_HANDLER(c, w, a) \
    void app_action_##c##_##w##_##a(void *vc, void *sender, void *event);

#define APP_ACTION_HANDLER_INVOKE(c, w, a, sender, event) \
    app_action_##c##_##w##_##a((__bridge void *)self, sender, event) \

#define UI_ACTION_HANDLER_IMPL(c, w, a) \
    - (IBAction) w##a:(id)sender forEvent:(UIEvent *)event { \
        app_action_##c##_##w##_##a((__bridge void *)self, (__bridge void *)sender, (__bridge void *)event); \
    }

UI_GET_PROPERTY(MainViewController, label1);
UI_GET_PROPERTY(MainViewController, textField1);
UI_GET_PROPERTY(MainViewController, footLabel);

APP_ACTION_HANDLER(MainViewController, self, Load);
APP_ACTION_HANDLER(MainViewController, button1, Tapped);
APP_ACTION_HANDLER(MainViewController, textField1, DidChange);
