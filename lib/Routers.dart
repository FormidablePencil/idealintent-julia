// import 'package:flutter/material.dart';
//
// const String SplashPath = '/splash';
// const String LoginPath = '/login';
// const String CreateAccountPath = '/createAccount';
// const String ListItemsPath = '/listItems';
// const String DetailsPath = '/details';
// const String CartPath = '/cart';
// const String CheckoutPath = '/checkout';
// const String SettingsPath = '/settings';
// enum Pages {
//   Splash,
//   Login,
//   CreateAccount,
//   List,
//   Details,
//   Cart,
//   Checkout,
//   Settings
// }
// class PageAction {
//   PageState state;
//   PageConfiguration page;
//   List<PageConfiguration> pages;
//   Widget widget;
//
//   PageAction({this.state = PageState.none, this.page = null, this.pages = null, this.widget = null});
// }
// class PageConfiguration {
//   final String key;
//   final String path;
//   final Pages uiPage;
//   PageAction currentPageAction;
//
//
//   PageConfiguration(
//       {@required this.key, @required this.path, @required this.uiPage, this.currentPageAction});
// }
// PageConfiguration SplashPageConfig =
// PageConfiguration(key: 'Splash', path: SplashPath, uiPage: Pages.Splash, currentPageAction: null);
// PageConfiguration LoginPageConfig =
// PageConfiguration(key: 'Login', path: LoginPath, uiPage: Pages.Login, currentPageAction: null);
// PageConfiguration CreateAccountPageConfig = PageConfiguration(
//     key: 'CreateAccount', path: CreateAccountPath, uiPage: Pages.CreateAccount, currentPageAction: null);
// PageConfiguration ListItemsPageConfig = PageConfiguration(
//     key: 'ListItems', path: ListItemsPath, uiPage: Pages.List);
// PageConfiguration DetailsPageConfig =
// PageConfiguration(key: 'Details', path: DetailsPath, uiPage: Pages.Details, currentPageAction: null);
// PageConfiguration CartPageConfig =
// PageConfiguration(key: 'Cart', path: CartPath, uiPage: Pages.Cart, currentPageAction: null);
// PageConfiguration CheckoutPageConfig = PageConfiguration(
//     key: 'Checkout', path: CheckoutPath, uiPage: Pages.Checkout, currentPageAction: null);
// PageConfiguration SettingsPageConfig = PageConfiguration(
//     key: 'Settings', path: SettingsPath, uiPage: Pages.Settings, currentPageAction: null);
//
//
// enum PageState {
//   none,
//   addPage,
//   addAll,
//   addWidget,
//   pop,
//   replace,
//   replaceAll
// }

