import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter/material.dart' hide Size;
import 'package:idealintent_julia/bridge_generated.dart';

// as api_bridge_generated;
// import 'package:idealintent_julia/compositions_api_bridge_generated.dart'
//     as compositions_api_bridge_generated;
import 'package:idealintent_julia/screens/BrowserScreen.dart';
import 'package:idealintent_julia/screens/PersonalPortfolioScreen.dart';
import 'package:idealintent_julia/ContentProvider.dart';
import 'package:provider/provider.dart';

// Simple Flutter code. If you are not familiar with Flutter, this may sounds a bit long. But indeed
// it is quite trivial and Flutter is just like that. Please refer to Flutter's tutorial to learn Flutter.

const base = 'idealintent_julia';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
late final dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
late final api = FlutterRustBridgeExampleImpl(dylib);
// late final apiCompositions =
//     compositions_api_bridge_generated.JuliaRustImpl(dylib);
// late final compositions_api = JuliaImpl(dylib);

void main() => runApp(
      MultiProvider(
        providers: [
          ChangeNotifierProvider(create: (_) => ContentProvider()),
        ],
        child: const MyApp(),
      ),
    );

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String? content;

  int currentIndex = 0;
  final screens = [BrowserScreen(), PersonalPortfolioScreen()];

  @override
  void initState() {
    super.initState();
    _getContent();
  }

  @override
  Widget build(BuildContext context) => MaterialApp(
      home: Scaffold(
          // appBar: AppBar(title: const Text("title"), centerTitle: true),
          body: IndexedStack(children: screens, index: currentIndex),
          bottomNavigationBar: BottomNavigationBar(
            type: BottomNavigationBarType.fixed,
            // backgroundColor: Color.blue,
            // selectedItemColor: Color.white,
            // unselectedItemColor: Color.white70,
            iconSize: 40,
            currentIndex: currentIndex,
            onTap: (idx) => setState(() => currentIndex = idx),
            items: const [
              BottomNavigationBarItem(icon: Icon(Icons.home), label: "Home"),
              BottomNavigationBarItem(icon: Icon(Icons.favorite), label: "Feed")
            ],
          )));

  Future<void> _getContent() async {
    // final receivedAddress =
    //     await api.upload(content: BasicParagraph(title: "title", body: "body"));
    // print(receivedAddress);
    // final receivedData = await api.get(address: receivedAddress);
    // print(receivedAddress);

// final d=      apiCompositions.testSuccess2();
//     print(d);
    // print(await apiCompositions.testSuccess2());
    // const compositionCategory =
    //     CompositionCategory.paragraph(ParagraphType.Basic);
    // final f = api.temp(compositionCategory: compositionCategory);
    final f = api.temp2();
    print(f);

// if (mounted) setState(() => content = receivedAddress);
    setState(() => content = "receivedAddress");
  }
}
