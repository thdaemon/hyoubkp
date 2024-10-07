#include "hmui.h"
#import <UIKit/UIKit.h>


char const *appui_string_cstr(void *s) {
    NSString *string = (__bridge NSString *) s;
    return [string UTF8String];
}

void appui_object_deref(void *o) {
    NSObject *_obj __attribute__((unused)) = (__bridge_transfer NSObject *) o;
}

void appui_uikit_control_set_enabled(void *control, int b) {
    UIControl *_self = (__bridge UIControl *)control;
    _self.enabled = b;
}

void appui_uikit_control_send_action(void *control, unsigned int event) {
    UIControl *_self = (__bridge UIControl *)control;
    [_self sendActionsForControlEvents:event];
}

void appui_uikit_label_set_text(void *label, char const *s) {
    UILabel *_self = (__bridge UILabel *)label;
    _self.text = [NSString stringWithUTF8String:s];
}

char const *appui_uikit_textField_get_text(void *textField) {
    UITextField *_self = (__bridge UITextField *)textField;
    return [_self.text UTF8String];
}

void appui_uikit_textField_set_text(void *textField, char const *s) {
    UITextField *_self = (__bridge UITextField *)textField;
    _self.text = [NSString stringWithUTF8String:s];
}

void appui_uikit_alertctrl(void *vc, char const *title, char const *message, void *callback, void *callback_userdata) {
    UIViewController *_self = (__bridge UIViewController *)vc;

    UIAlertController* alert = [UIAlertController alertControllerWithTitle:[NSString stringWithUTF8String:title]
                               message:[NSString stringWithUTF8String:message]
                               preferredStyle:UIAlertControllerStyleAlert];
 
    UIAlertAction* defaultAction = [UIAlertAction actionWithTitle:@"OK" style:UIAlertActionStyleDefault
        handler:^(UIAlertAction *action) {
            if (callback) {
                void (*fn)(void *action, void *callback_userdata) = callback;
                fn((__bridge void *)action, callback_userdata);
            }
        }
    ];
 
    [alert addAction:defaultAction];
    [_self presentViewController:alert animated:YES completion:nil];
}

void appui_uikit_viewcontroller_set_modalPresentationStyle(void *vc, int value) {
    UIViewController *_self = (__bridge UIViewController *)vc;
    _self.modalPresentationStyle = value;
}

void appui_uikit_viewcontroller_presentViewController(void *vc, void *vc_to_present, bool animated, void *_unimplemented_callback, void *_unimplemented_callback_userdata) {
    UIViewController *_self = (__bridge UIViewController *)vc;
    UIViewController *to_present = (__bridge UIViewController *)vc_to_present;
    [_self presentViewController:to_present animated:animated completion:nil];
}

void *appui_fs_document_path() {
    NSArray *paths = NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSUserDomainMask, YES);
    NSString *documentsDirectory = [paths firstObject];
    return (__bridge_retained void *)documentsDirectory;
}

bool appui_userdefaults_fsync() {
    return (bool)[[NSUserDefaults standardUserDefaults] synchronize];
}

void appui_userdefaults_set_i32(char const *key, int32_t i) {
    [[NSUserDefaults standardUserDefaults] setInteger:i forKey:[NSString stringWithUTF8String:key]];
}

int32_t appui_userdefaults_get_i32(char const *key) {
    return (int32_t)[[NSUserDefaults standardUserDefaults] integerForKey:[NSString stringWithUTF8String:key]];
}

void appui_userdefaults_set_string(char const *key, char const *s) {
    NSString *string = [NSString stringWithUTF8String:s];
    [[NSUserDefaults standardUserDefaults] setObject:string forKey:[NSString stringWithUTF8String:key]];
}

void *appui_userdefaults_get_string(char const *key) {
    return (__bridge_retained void *)[[NSUserDefaults standardUserDefaults] stringForKey:[NSString stringWithUTF8String:key]];
}

void *appui_nsurl_new_fileURLWithPath(char const *path) {
    NSString *filePath = [NSString stringWithUTF8String:path];
    NSURL *fileURL = [NSURL fileURLWithPath:filePath];
    return (__bridge_retained void *)fileURL;
}