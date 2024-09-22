#include "hmui.h"
#import <UIKit/UIKit.h>


void appui_uikit_control_set_enabled(void *control, int b) {
    UIControl *_self = (__bridge UIControl *)control;
    _self.enabled = b;
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

char const *appui_fs_document_path() {
    NSArray *paths = NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSUserDomainMask, YES);
    NSString *documentsDirectory = [paths firstObject];
    return (__bridge void *)[documentsDirectory UTF8String];
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