import 'package:fluster_media_center/features/LibraryFilters/widgets/placeholder_banner.dart';
import 'package:flutter/material.dart';

class LibraryHeader extends StatelessWidget {
  const LibraryHeader({super.key});

  @override
  Widget build(BuildContext context) {
    return SliverToBoxAdapter(
      child: Container(
        height: 500,
        decoration: BoxDecoration(
          color: Color.fromARGB(255, 200, 220, 220),
          borderRadius: BorderRadius.all(Radius.circular(12)),
        ),
        child: PlaceholderBanner(),
      ),
    );
  }
}
