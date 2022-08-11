import 'package:flutter/foundation.dart';
import 'package:idealintent_julia/compositions/MyCardData.dart';
import 'package:idealintent_julia/compositions/MyExpansionPanelList.dart';

/// Mix-in [DiagnosticableTreeMixin] to have access to [debugFillProperties] for the devtool
// ignore: prefer_mixin
class ContentProvider with ChangeNotifier, DiagnosticableTreeMixin {
  MyCardData _myCardData = const MyCardData(
    title: 'The Enchanted Nightingale',
    subtitle: 'Flutter Code Sample',
    button2: 'Buy tickets',
    button1: 'Listen',
  );

  MyCardData get myCardData => _myCardData;

  void myCardDataChange(MyCardData data) {
    _myCardData = data;
    notifyListeners();
  }

  // void myCardDataChange(title, subtitle, button2, button1) {
  //   MyCardData(
  //     title: title,
  //     subtitle: subtitle,
  //     button2: button2,
  //     button1: button1,
  //   );
  // }

  final _myExpansionPanelList = [
    Item(
      headerValue: 'Panel 1',
      expandedValue: 'This is item number 1',
    )
  ];

  List<Item> get myExpansionPanelList => _myExpansionPanelList;

  void increment() {
    // _myCardData++;
    notifyListeners();
  }

  void addToMyExpansionPanelList(Item item) {
    // _myCardData++;
    _myExpansionPanelList.add(item);
    notifyListeners();
  }

  /// Makes `Counter` readable inside the devtools by listing all of its properties
// @override
// void debugFillProperties(DiagnosticPropertiesBuilder properties) {
//   super.debugFillProperties(properties);
//   properties.add(MessageProperty('myCardData', myCardData));
// }
}
