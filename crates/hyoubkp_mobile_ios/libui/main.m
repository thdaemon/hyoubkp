#include "hmui.h"
#import <UIKit/UIKit.h>
#import "MainViewController.h"

@interface AppDelegate : UIResponder <UIApplicationDelegate>

@property (strong, nonatomic) UIWindow *window;

@end

@implementation AppDelegate

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    self.window = [[UIWindow alloc] initWithFrame:[[UIScreen mainScreen] bounds]];

    UINavigationController *navController = [[UINavigationController alloc] initWithRootViewController:[[MainViewController alloc] init]];
    
    self.window.rootViewController = navController;
    [self.window makeKeyAndVisible];
    return YES;
}

@end

int appui_main(int argc, char *argv[]) {
    @autoreleasepool {
	    NSLog(@"hello!!!\n");
        return UIApplicationMain(argc, argv, nil, NSStringFromClass([AppDelegate class]));
    }
}
