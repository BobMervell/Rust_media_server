import 'package:flutter/material.dart';
import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:fluster_media_center/src/rust/frb_generated.dart';
import 'package:fluster_media_center/features/NavigatorRail/screens/NavigatorRail.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        scaffoldBackgroundColor: Color.fromARGB(
          255,
          160,
          180,
          180,
        ), // fond global
      ),
      home: Scaffold(
        body: Row(
          children: [
            Expanded(flex: 1, child: NavigatorRail()),
            Expanded(flex: 4, child: TempoHome()),
          ],
        ),
      ),
    );
  }
}

class TempoHome extends StatelessWidget {
  const TempoHome({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(child: Text("how r u"));
  }
}

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

// TODO create a widget for movie snapshot (button mode)
//
//TODO create a gridview with movies
//
//TODO create detailed movie template
//
//TODO create filters row

//TODO first version finished init ?
