import 'package:fluster_media_center/features/LibraryViewer/widgets/library_filters.dart';
import 'package:fluster_media_center/features/LibraryViewer/widgets/library_header.dart';
import 'package:fluster_media_center/features/LibraryViewer/widgets/library_view.dart';
import 'package:flutter/material.dart';

class Library extends StatelessWidget {
  const Library({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
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
    );
  }
}
