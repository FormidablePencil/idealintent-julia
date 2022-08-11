import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';

class MyQuillText extends StatelessWidget {
  MyQuillText({Key? key}) : super(key: key);
  final QuillController _controller = QuillController.basic();

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        QuillToolbar.basic(controller: _controller),
        Expanded(
          child: Container(
            child: QuillEditor.basic(
              controller: _controller,
              readOnly: false, // true for view only mode
            ),
          ),
        )
      ],
    );
  }
}
