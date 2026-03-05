import 'package:fluster_media_center/features/LibraryFilters/screens/library_filters.dart';
import 'package:flutter/material.dart';
import 'package:fluster_media_center/src/rust/frb_generated.dart';
import 'package:fluster_media_center/features/NavigatorRail/screens/navigator_rail.dart';
import 'package:fluster_media_center/features/LibraryView/screens/library_view.dart';
import 'package:fluster_media_center/features/LibraryHeader/screens/library_header.dart';

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
            Expanded(
              flex: 4,
              child: Padding(
                padding: const EdgeInsets.all(20.0),
                child: CustomScrollView(
                  slivers: [
                    LibraryHeader(),
                    SliverToBoxAdapter(child: SizedBox(height: 20)),
                    LibraryFilters(),
                    SliverToBoxAdapter(child: SizedBox(height: 20)),
                    LibraryView(),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
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
