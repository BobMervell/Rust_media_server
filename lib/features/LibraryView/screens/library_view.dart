import 'package:fluster_media_center/features/LibraryView/widgets/item_snapshot.dart';
import 'package:flutter/material.dart';

class LibraryView extends StatelessWidget {
  const LibraryView({super.key});

  @override
  Widget build(BuildContext context) {
    return SliverGrid.builder(
      gridDelegate: const SliverGridDelegateWithMaxCrossAxisExtent(
        maxCrossAxisExtent: 200,
        mainAxisSpacing: 100,
        crossAxisSpacing: 20,
        childAspectRatio: 1,
      ),
      itemCount: null,
      itemBuilder: (BuildContext context, int index) {
        return ItemSnapshot(name: 'Item $index');
      },
    );
  }
}
