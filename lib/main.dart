import 'package:fluster_media_center/features/LibraryFilters/screens/library.dart';
import 'package:flutter/material.dart';
import 'package:fluster_media_center/src/rust/frb_generated.dart';
import 'package:fluster_media_center/features/NavigatorRail/screens/navigator_rail.dart';
import 'package:media_kit/media_kit.dart';

Future<void> main() async {
  await RustLib.init();
  MediaKit.ensureInitialized(); // Necessary initialization for package:media_kit.
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
            Expanded(flex: 4, child: Library()),
          ],
        ),
      ),
    );
  }
}
