#include "hmui.h"
#import "MainViewController.h"

@interface MainViewController ()

@property (nonatomic, strong) UIScrollView *scrollView;
@property (nonatomic, strong) UIView *contentView;
@property (nonatomic, strong) UITextField *textField1;
@property (nonatomic, strong) UIButton *button1;
@property (nonatomic, strong) UIButton *button2;
@property (nonatomic, strong) UILabel *label1;

@end

@implementation MainViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    
    self.view.backgroundColor = [UIColor whiteColor];
    self.title = @"Hyoubkp Mobile";

    UIImage *menuImage = [UIImage systemImageNamed:@"ellipsis.circle"];
    UIBarButtonItem *menuButton = [[UIBarButtonItem alloc] initWithImage:menuImage style:UIBarButtonItemStylePlain target:self action:nil];
    UIAction *option1 = [UIAction actionWithTitle:@"Not implemented yet" image:nil identifier:nil handler:^(__kindof UIAction * _Nonnull action) {
    }];
    
    UIMenu *menu = [UIMenu menuWithTitle:@"" children:@[option1]];
    menuButton.menu = menu;

    self.navigationItem.rightBarButtonItems = @[menuButton];

    self.textField1 = [[UITextField alloc] init];
    self.textField1.borderStyle = UITextBorderStyleRoundedRect;
    self.textField1.placeholder = @"Enter the expression";
    self.textField1.translatesAutoresizingMaskIntoConstraints = NO;
    [self.textField1 addTarget:self action:@selector(textField1DidChange:forEvent:) forControlEvents:UIControlEventEditingChanged];
    [self.view addSubview:self.textField1];
    
    self.button1 = [UIButton buttonWithType:UIButtonTypeSystem];
    [self.button1 setTitle:@"Commit" forState:UIControlStateNormal];
    [self.button1 addTarget:self action:@selector(button1Tapped:forEvent:) forControlEvents:UIControlEventTouchUpInside];
    self.button1.translatesAutoresizingMaskIntoConstraints = NO;
    [self.view addSubview:self.button1];
    
    self.scrollView = [[UIScrollView alloc] init];
    self.scrollView.translatesAutoresizingMaskIntoConstraints = NO;
    [self.view addSubview:self.scrollView];
    
    self.contentView = [[UIView alloc] init];
    self.contentView.translatesAutoresizingMaskIntoConstraints = NO;
    [self.scrollView addSubview:self.contentView];

    self.label1 = [[UILabel alloc] init];
    self.label1.text = @"[]";
    self.label1.translatesAutoresizingMaskIntoConstraints = NO;
    self.label1.numberOfLines = 0;
    [self.contentView addSubview:self.label1];

    UITapGestureRecognizer *tapGesture = [[UITapGestureRecognizer alloc] initWithTarget:self action:@selector(dismissKeyboard)];
    [self.view addGestureRecognizer:tapGesture];

    [self setupConstraints];

    [[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(keyboardWillShow:) name:UIKeyboardWillShowNotification object:nil];
    [[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(keyboardWillHide:) name:UIKeyboardWillHideNotification object:nil];

    APP_ACTION_HANDLER_INVOKE(MainViewController, self, Load, nil, nil);
}

- (void)setupConstraints {
    UILayoutGuide *safeArea = self.view.safeAreaLayoutGuide;
    
    [NSLayoutConstraint activateConstraints:@[
        [self.textField1.topAnchor constraintEqualToAnchor:safeArea.topAnchor constant:15],
        [self.textField1.leadingAnchor constraintEqualToAnchor:safeArea.leadingAnchor constant:15],
        [self.textField1.trailingAnchor constraintEqualToAnchor:safeArea.trailingAnchor constant:-15-80],
        
        [self.button1.centerYAnchor constraintEqualToAnchor:self.textField1.centerYAnchor],
        [self.button1.trailingAnchor constraintEqualToAnchor:safeArea.trailingAnchor constant:-15],
        [self.button1.widthAnchor constraintEqualToConstant:80]
    ]];

    [NSLayoutConstraint activateConstraints:@[
        [self.scrollView.topAnchor constraintEqualToAnchor:self.button1.bottomAnchor constant:15],
        [self.scrollView.leadingAnchor constraintEqualToAnchor:safeArea.leadingAnchor],
        [self.scrollView.trailingAnchor constraintEqualToAnchor:safeArea.trailingAnchor],
        [self.scrollView.bottomAnchor constraintEqualToAnchor:safeArea.bottomAnchor]
    ]];
    
    [NSLayoutConstraint activateConstraints:@[
        [self.contentView.topAnchor constraintEqualToAnchor:self.scrollView.topAnchor],
        [self.contentView.leadingAnchor constraintEqualToAnchor:self.scrollView.leadingAnchor],
        [self.contentView.trailingAnchor constraintEqualToAnchor:self.scrollView.trailingAnchor],
        [self.contentView.bottomAnchor constraintEqualToAnchor:self.scrollView.bottomAnchor],
        [self.contentView.widthAnchor constraintEqualToAnchor:self.view.widthAnchor]
    ]];

    [NSLayoutConstraint activateConstraints:@[
        [self.label1.topAnchor constraintEqualToAnchor:self.contentView.topAnchor],
        [self.label1.leadingAnchor constraintEqualToAnchor:self.contentView.leadingAnchor constant:15],
        [self.label1.trailingAnchor constraintEqualToAnchor:self.contentView.trailingAnchor constant:-15],
        [self.label1.bottomAnchor constraintEqualToAnchor:self.contentView.bottomAnchor constant:-15]
    ]];
}

- (void)keyboardWillShow:(NSNotification *)notification {
    NSDictionary *userInfo = notification.userInfo;
    CGRect keyboardFrame = [userInfo[UIKeyboardFrameEndUserInfoKey] CGRectValue];
    
    UIEdgeInsets contentInsets = UIEdgeInsetsMake(0, 0, keyboardFrame.size.height, 0);
    self.scrollView.contentInset = contentInsets;
    self.scrollView.scrollIndicatorInsets = contentInsets;
}

- (void)keyboardWillHide:(NSNotification *)notification {
    UIEdgeInsets contentInsets = UIEdgeInsetsZero;
    self.scrollView.contentInset = contentInsets;
    self.scrollView.scrollIndicatorInsets = contentInsets;
}

- (void)dismissKeyboard {
    [self.view endEditing:YES];
}

- (void)dealloc {
    [[NSNotificationCenter defaultCenter] removeObserver:self];
    [super dealloc];
}

UI_ACTION_HANDLER_IMPL(MainViewController, button1, Tapped)
UI_ACTION_HANDLER_IMPL(MainViewController, textField1, DidChange)

@end

UI_GET_PROPERTY_IMPL(MainViewController, label1);
UI_GET_PROPERTY_IMPL(MainViewController, textField1);
