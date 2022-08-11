import 'package:flutter/material.dart';
import 'package:idealintent_julia/ContentProvider.dart';
import 'package:idealintent_julia/compositions/MyCardData.dart';
import 'package:provider/provider.dart';

// stores ExpansionPanel state information
class Item {
  Item({
    required this.expandedValue,
    required this.headerValue,
    this.isExpanded = false,
  });

  String expandedValue;
  String headerValue;
  bool isExpanded;
}

List<Item> generateItems(int numberOfItems) {
  return List<Item>.generate(numberOfItems, (int index) {
    return Item(
      headerValue: 'Panel $index',
      expandedValue: 'This is item number $index',
    );
  });
}

class MyExpansionPanelList extends StatefulWidget {
  const MyExpansionPanelList({Key? key, required this.data}) : super(key: key);

  final List<Item> data;

  @override
  State<MyExpansionPanelList> createState() => _MyExpansionPanelList();
}

class _MyExpansionPanelList extends State<MyExpansionPanelList> {
  @override
  Widget build(BuildContext context) {
    MyCardData myCardData = Provider.of<ContentProvider>(context).myCardData;
    void Function(MyCardData) myCardDataChange = Provider.of<ContentProvider>(context).myCardDataChange;

    return ExpansionPanelList(
      expansionCallback: (int index, bool isExpanded) {
        setState(() {
          widget.data[index].isExpanded = !isExpanded;
        });
      },
      children: widget.data.map<ExpansionPanel>((Item item) {
        return ExpansionPanel(
          headerBuilder: (BuildContext context, bool isExpanded) {
            return ListTile(
              title: Text(item.headerValue),
            );
          },
          body: ListTile(
              title: Text(item.expandedValue),
              subtitle:
                  const Text('To delete this panel, tap the trash can icon'),
              trailing: IconButton(
                icon: const Icon(Icons.delete),
                onPressed: () {
                  setState(() {
                    widget.data
                        .removeWhere((Item currentItem) => item == currentItem);
                  });
                },
              ),
              onTap: () {
                  myCardDataChange(MyCardData(
                  title: item.headerValue,
                  subtitle: item.expandedValue,
                  button2: 'Buy tickets',
                  button1: 'Listen',
                ));
                //   widget.data.removeWhere((Item currentItem) => item == currentItem);
              }),
          isExpanded: item.isExpanded,
        );
      }).toList(),
    );
  }
}
