/// Code in this file is more about Flutter than our package, `flutter rust bridge`.
/// To understand this package, you do not need to have a deep understanding of this file and Flutter.
/// Thus, we put it in this "utility" file, instead of the "main" file.
import 'dart:async';
import 'dart:math';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:idealintent_julia/bridge_generated.dart';
import 'package:idealintent_julia/text_exper.dart';

Widget buildPageUi(String? content /*Uint8List? exampleImage, String? exampleText*/) {
  final myController = TextEditingController();

  return MaterialApp(
    home: Scaffold(
      appBar: AppBar(title: const Text('Flutter Rust Bridge Example')),
      body: ListView(
        children: [
          Container(height: 16),
          // Container(
          //   padding: const EdgeInsets.symmetric(horizontal: 24),
          //   child: Card(
          //     child: Container(
          //       padding:
          //           const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
          //       child: Column(
          //         children: [
          //           const Text('Example 1',
          //               style: TextStyle(
          //                   fontSize: 18, fontWeight: FontWeight.bold)),
          //           Container(height: 8),
          //           const Text(
          //               'Image generated (periodically) by Rust and displayed by Flutter/Dart'),
          //           Container(height: 24),
          //           (exampleImage != null
          //               ? SizedBox(
          //                   width: 50,
          //                   height: 50,
          //                   child: Center(
          //                       child: AnimatedReplaceableImage(
          //                           image: MemoryImage(exampleImage))))
          //               : Container()),
          //           Container(height: 4),
          //           const Text('Mandelbrot Set',
          //               style: TextStyle(fontSize: 11, color: Colors.grey)),
          //           const Text('classical image requiring lots of computing',
          //               style: TextStyle(fontSize: 11, color: Colors.grey)),
          //           Container(height: 8),
          //         ],
          //       ),
          //     ),
          //   ),
          // ),
          Container(height: 16),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 24),
            child: Card(
              child: Container(
                padding:
                    const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                child: Column(
                  children: [
                    const Text('Example 2',
                        style: TextStyle(
                            fontSize: 18, fontWeight: FontWeight.bold)),
                    Container(height: 8),
                    const Text(
                        'Complex struct/class is passed smoothly through FFI'),
                    Container(height: 24),
                    Text(/*exampleText*/ content ?? 'empty',
                        style:
                            const TextStyle(fontSize: 11, color: Colors.grey)),
                    Container(height: 8),
                  ],
                ),
              ),
            ),
          ),
          BasicTextComposition(myController: myController),
        ],
      ),
    ),
  );
}

class AnimatedReplaceableImage extends StatefulWidget {
  final ImageProvider image;

  const AnimatedReplaceableImage({Key? key, required this.image})
      : super(key: key);

  @override
  _AnimatedReplaceableImageState createState() =>
      _AnimatedReplaceableImageState();
}

class _AnimatedReplaceableImageState extends State<AnimatedReplaceableImage> {
  ImageProvider? previousImage;

  @override
  void initState() {
    super.initState();
    previousImage = null;
  }

  @override
  void didUpdateWidget(AnimatedReplaceableImage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.image.obtainKey(const ImageConfiguration()) !=
        widget.image.obtainKey(const ImageConfiguration())) {
      previousImage = oldWidget.image;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Image(
      image: widget.image,
      frameBuilder: (BuildContext context, Widget child, int? frame,
              bool wasSynchronouslyLoaded) =>
          (frame == null && previousImage != null)
              ? Stack(children: [Image(image: previousImage!), child])
              : child,
    );
  }
}

double _scale = 1.0;

double generateScale() {
  _scale *= 0.5;
  if (_scale < 1e-9) _scale = 1.0;
  return _scale;
}

void runPeriodically(void Function() callback) =>
    Timer.periodic(const Duration(milliseconds: 500), (timer) => callback());

class MyWidget extends StatelessWidget {
  MyWidget();
  MyWidget.forDesignTime();

  @override
  Widget build(BuildContext context) {
    return new Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      children: <Widget>[
        new Text('A'),
        new Text('BB'),
        new Text('CCC'),
      ],
    );
  }
}