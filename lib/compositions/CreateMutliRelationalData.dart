import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:idealintent_julia/ContentProvider.dart';
import 'package:idealintent_julia/compositions/MyExpansionPanelList.dart';
import 'package:provider/provider.dart';

class CreateMutliRelationalData extends StatelessWidget {
  CreateMutliRelationalData({Key? key}) : super(key: key);
  final inputTitle = TextEditingController();
  final inputDescription = TextEditingController();
  final inputRelationship = TextEditingController();

  @override
  Widget build(BuildContext context) {
    void Function(Item item) addToMyExpansionPanelList =
        Provider.of<ContentProvider>(context).addToMyExpansionPanelList;
    final _formKey = GlobalKey<FormState>();

    return Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: <Widget>[
          SizedBox(
            width: 300,
            child: Column(children: <Widget>[
              Form(
                key: _formKey,
                child: Column(
                  children: <Widget>[
                    TextFormField(
                      controller: inputTitle,
                      decoration: const InputDecoration(
                        hintText: "Title",
                        filled: true,
                        fillColor: Colors.blueGrey,
                      ),
                    ),

                    TextFormField(
                      controller: inputDescription,
                      decoration: const InputDecoration(
                        hintText: "Description",
                        filled: true,
                        fillColor: Colors.blueGrey,
                      ),
                    ),

                    TextFormField(
                      controller: inputRelationship,
                      decoration: const InputDecoration(
                        hintText: "Relationship - todo",
                        filled: true,
                        fillColor: Colors.blueGrey,
                      ),
                      validator: (value) {
                        // if (value == null || value.isEmpty) {
                        //   return 'Please enter some text';
                        // }
                        // return null;
                      },
                    ),
                    // Add TextFormFields and ElevatedButton here.
                  ],
                ),
              ),
              Container(
                width: 300,
                color: Colors.amberAccent,
                child: FlatButton(
                  child: const Text('Create data'),
                  onPressed: () {
                    addToMyExpansionPanelList(Item(
                      expandedValue: inputTitle.text,
                      headerValue: inputDescription.text,
                      isExpanded: true,
                    ));
                    debugPrint('inputTitle: ${inputTitle.text}');
                    debugPrint('inputDescription: ${inputDescription.text}');
                    debugPrint('inputRelationship: ${inputRelationship.text}');
                  },
                ),
              )
            ]),
          ),
        ]);
  }
}
