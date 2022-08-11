import 'package:flutter/material.dart';
import 'package:idealintent_julia/compositions/texts/TextBasic.dart';

import '../bridge_generated.dart';

class CompositionLayout extends StatefulWidget {
  const CompositionLayout({Key? key}) : super(key: key);

  @override
  State<CompositionLayout> createState() => _CompositionLayoutState();
}

class _CompositionLayoutState extends State<CompositionLayout> {
  // const compositionCategory = ;
  List<DataWrapper> litems = [
    DataWrapper(data: "", metadata: '', dataType: '')
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: ListView.builder(
            itemCount: litems.length,
            itemBuilder: (BuildContext ctxt, int index) {
              return DynamicComposition(litems);
            }));
  }
}

Widget DynamicComposition(List<DataWrapper> listDataWrapper) {
  // const DynamicComposition(,
  //     {Key? key})
  //     : super(key: key);

  _getComposition() {
    // const compositionCategory = CompositionCategory.text(TextType.Basic);
    // todo - convert dataWrapper
    final f = listDataWrapper;

    buildTextBasic(BasicParagraph(title: "title", body: "body"));

    // compositionCategory.when(
    //     carousel: (CarouselType field0) => TODO(),
    //     banner: (BannerType field0) => TODO(),
    //     text: (TextType field0) {
    //       switch (field0) {
    //         case TextType.Basic:
    //           buildTextBasic(BasicText(title: "title", body: "body"));
    //       }
    //     });

    return buildTextBasic(BasicParagraph(title: "title", body: "body"));
  }

  return _getComposition();
}
