import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart' hide Size;
import 'package:idealintent_julia/bridge_generated.dart';
import 'package:idealintent_julia/off_topic_code.dart';

// Simple Flutter code. If you are not familiar with Flutter, this may sounds a bit long. But indeed
// it is quite trivial and Flutter is just like that. Please refer to Flutter's tutorial to learn Flutter.

const base = 'flutter_rust_bridge_example';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
late final dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
late final api = FlutterRustBridgeExampleImpl(dylib);

void main() => runApp(const MyApp());

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String? content;

  // Uint8List? exampleImage;
  // String? exampleText;

  @override
  void initState() {
    super.initState();
    // runPeriodically(_callExampleFfiOne);
    // _callExampleFfiTwo();
    _get_content();
  }

  @override
  Widget build(BuildContext context) => buildPageUi(
        // todo - only one for now but will be a list of content based on spaces/object's addresses associated to some address queried for
        content,
        // exampleImage,
        // exampleText,
      );

  Future<void> _get_content() async {

    final receivedAddress =
    await api.upload(content: BasicText(title: "title", body: "body"));
    final receivedData = await api.get(address: receivedAddress);
    print(receivedAddress);
    //
    // final receivedAddress1 =
    // await api.upload1(content: BasicText(title: "title", body: "body"));
    // print(receivedAddress1);
    //
    // final receivedAddress =
    //     await api.upload(content: BasicText(title: "title", body: "body"));
    // print(receivedAddress);
    // final receivedContent = await api.get(address: receivedAddress);
    // print(receivedContent);

    // if (mounted) setState(() => content = receivedAddress);
    setState(() => content = receivedAddress);
  }

// Future<void> _callExampleFfiOne() async {
//   final receivedImage = await api.drawMandelbrot(
//       imageSize: Size(width: 50, height: 50),
//       zoomPoint: examplePoint,
//       scale: generateScale(),
//       numThreads: 4);
//   if (mounted) setState(() => exampleImage = receivedImage);
// }
//
// Future<void> _callExampleFfiTwo() async {
//   final receivedText =
//       await api.passingComplexStructs(root: createExampleTree());
//   if (mounted) setState(() => exampleText = receivedText);
// }
}
