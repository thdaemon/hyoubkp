#pragma once

#include <stdint.h>
#include <stdbool.h>

int appui_main(int argc, char *argv[]);

char const *appui_string_cstr(void *s);
void appui_object_deref(void *o);

void appui_uikit_control_set_enabled(void *control, int b);
void appui_uikit_control_send_action(void *control, unsigned int event);
void appui_uikit_label_set_text(void *label, char const *s);
char const *appui_uikit_textField_get_text(void *textField);
void appui_uikit_textField_set_text(void *textField, char const *s);

void appui_uikit_alertctrl(void *vc, char const *title, char const *message, void *callback, void *callback_userdata);

void appui_uikit_viewcontroller_set_modalPresentationStyle(void *vc, int value);
void appui_uikit_viewcontroller_presentViewController(void *vc, void *vc_to_present, bool animated, void *_unimplemented_callback, void *_unimplemented_callback_userdata);

void *appui_fs_document_path();

bool appui_userdefaults_fsync();
void appui_userdefaults_set_i32(char const *key, int32_t i);
int32_t appui_userdefaults_get_i32(char const *key);
void appui_userdefaults_set_string(char const *key, char const *s);
void *appui_userdefaults_get_string(char const *key);

void *appui_nsurl_new_fileURLWithPath(char const *path);

#define UI_GET_PROPERTY(c, p) \
    void *appui_##c##_##p(void *obj);

#define UI_GET_PROPERTY_IMPL(c, p) \
    void *appui_##c##_##p(void *obj) { \
        c *o = (__bridge c *)obj; \
        return (__bridge void *)o.p; \
    }

#define UI_SET_PROPERTY(c, p) \
    void appui_##c##_##p##_set_bridge(void *obj, void *v); \
    void appui_##c##_##p##_set_transfer(void *obj, void *v);

#define UI_SET_PROPERTY_IMPL(c, p, t) \
    void appui_##c##_##p##_set_bridge(void *obj, void *v) { \
        c *o = (__bridge c *)obj; \
        o.p = (__bridge t *)v; \
    } \
    void appui_##c##_##p##_set_transfer(void *obj, void *v) { \
        c *o = (__bridge c *)obj; \
        o.p = (__bridge_transfer t *)v; \
    }

#define UI_NEW(c) \
    void *appui_##c##_new();

#define UI_NEW_IMPL(c) \
    void *appui_##c##_new() { \
        c *o = [[c alloc] init]; \
        return (__bridge_retained void *)o; \
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

UI_NEW(WebViewController);
UI_GET_PROPERTY(WebViewController, mainURL);
UI_SET_PROPERTY(WebViewController, mainURL);

APP_ACTION_HANDLER(MainViewController, self, Load);
APP_ACTION_HANDLER(MainViewController, menu1, Tapped);
APP_ACTION_HANDLER(MainViewController, button1, Tapped);
APP_ACTION_HANDLER(MainViewController, textField1, DidChange);
