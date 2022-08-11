import 'package:flutter/material.dart';
import 'package:idealintent_julia/ContentProvider.dart';
import 'package:idealintent_julia/compositions/CreateMutliRelationalData.dart';
import 'package:idealintent_julia/compositions/MyCard.dart';
import 'package:provider/provider.dart';

import '../compositions/MyCardData.dart';
import '../compositions/MyExpansionPanelList.dart';

// 0. save data locally and back it up some place or something
// - 1. click on expanded item and expand/open it
// 2. create/attach relationship
// 3. select a list of options of relationship and to what content
class BrowserScreen extends StatefulWidget {
  bool isSwitched = false;

  @override
  State<BrowserScreen> createState() => _BrowserScreenState();
}

class _BrowserScreenState extends State<BrowserScreen> {
  @override
  Widget build(BuildContext context) {
    MyCardData myCardData = Provider.of<ContentProvider>(context).myCardData;
    List<Item> myExpansionPanelList =
        Provider.of<ContentProvider>(context).myExpansionPanelList;

    final _formKey = GlobalKey<FormState>();

    toggleSwitch() {
      setState(() {
        widget.isSwitched = !widget.isSwitched;
      });
    }

    return Scaffold(
        body: Stack(fit: StackFit.expand, children: <Widget>[
      ListView(children: [
        MyCard(
          myCardData: myCardData,
          isSwitched: widget.isSwitched,
          toggleSwitch: toggleSwitch,
        ),
        MyExpansionPanelList(data: myExpansionPanelList),
        // MyQuillText()
      ]),
      if (widget.isSwitched == true) CreateMutliRelationalData(),
    ]));
  }
}

class DetailScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: FlatButton(
          child: const Text('Pop!'),
          onPressed: () {
            Navigator.pop(context);
          },
        ),
      ),
    );
  }
}
