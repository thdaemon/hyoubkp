#include "hmui.h"
#include "WebViewController.h"
#import <WebKit/WebKit.h>

@interface WebViewController ()

@property (nonatomic, strong) UINavigationBar *navBar;
@property (nonatomic, strong) WKWebView *webView;

@end

@implementation WebViewController

- (void)viewDidLoad {
    [super viewDidLoad];

    self.navBar = [[UINavigationBar alloc] initWithFrame:CGRectMake(0, 0, self.view.frame.size.width, 44)];
    UINavigationItem *navItem = [[UINavigationItem alloc] init];

    UIBarButtonItem *closeButton = [[UIBarButtonItem alloc] initWithTitle:@"Close" style:UIBarButtonItemStylePlain target:self action:@selector(closeModalView)];
    navItem.leftBarButtonItem = closeButton;
    UIBarButtonItem *shareButton = [[UIBarButtonItem alloc] initWithTitle:@"Share" style:UIBarButtonItemStylePlain target:self action:@selector(shareFile)];
    navItem.rightBarButtonItem = shareButton;

    self.navBar.items = @[navItem];
    [self.view addSubview:self.navBar];

    CGFloat navBarHeight = CGRectGetHeight(self.navBar.frame);
    self.webView = [[WKWebView alloc] initWithFrame:CGRectMake(0, navBarHeight, self.view.frame.size.width, self.view.frame.size.height - navBarHeight)];
    self.webView.autoresizingMask = UIViewAutoresizingFlexibleWidth | UIViewAutoresizingFlexibleHeight;
    [self.view addSubview:self.webView];

    [self.webView loadFileURL:self.mainURL allowingReadAccessToURL:self.mainURL];
}

- (void)closeModalView {
    [self dismissViewControllerAnimated:YES completion:nil];
}

- (void)shareFile {
    UIActivityViewController *activityVC = [[UIActivityViewController alloc] initWithActivityItems:@[self.mainURL] applicationActivities:nil];
    [self presentViewController:activityVC animated:YES completion:nil];
}

@end

UI_NEW_IMPL(WebViewController);

UI_GET_PROPERTY_IMPL(WebViewController, mainURL);
UI_SET_PROPERTY_IMPL(WebViewController, mainURL, NSURL);