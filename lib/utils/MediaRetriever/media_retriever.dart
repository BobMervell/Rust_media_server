import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:flutter/material.dart';

class MediaRetriever extends StatelessWidget {
  const MediaRetriever({super.key});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<String>(
      future: start(path: "", username: "", password: "", token: ""),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const CircularProgressIndicator();
        } else if (snapshot.hasError) {
          return Text('Error: ${snapshot.error}');
        } else {
          return Text('Action: Call Rust `start()`\nResult: ${snapshot.data}');
        }
      },
    );
  }
}
