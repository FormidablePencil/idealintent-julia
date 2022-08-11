import 'package:flutter/material.dart';
import 'package:idealintent_julia/compositions/MyCardData.dart';

class MyCard extends StatefulWidget {
  final MyCardData myCardData;
  final bool isSwitched;

  final toggleSwitch;

  const MyCard(
      {Key? key,
      required this.myCardData,
      required this.isSwitched,
      required this.toggleSwitch})
      : super(key: key);

  @override
  State<MyCard> createState() => _MyCardState();
}

class _MyCardState extends State<MyCard> {
  // static const String _title = 'Flutter Code Sample';
  @override
  Widget build(BuildContext context) {
    return Card(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: <Widget>[
          ListTile(
            leading: const Icon(Icons.album),
            title: Text(widget.myCardData.title),
            subtitle: Text(widget.myCardData.subtitle),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: <Widget>[
              TextButton(
                child: const Text('Add multi relational data'),
                onPressed: () {
                  setState(() {
                    widget.toggleSwitch();
                  });
                },
              ),
              const SizedBox(width: 8),
              TextButton(
                child: const Text('Edit'),
                onPressed: () {
                  /* ... */
                },
              ),
              const SizedBox(width: 8),
            ],
          ),
        ],
      ),
    );
  }
}
