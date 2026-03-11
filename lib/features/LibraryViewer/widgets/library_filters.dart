import 'package:fluster_media_center/features/LibraryViewer/widgets/filter_snapshot.dart';
import 'package:flutter/material.dart';

class LibraryFilters extends StatelessWidget {
  const LibraryFilters({super.key});

  @override
  Widget build(BuildContext context) {
    double width = MediaQuery.of(context).size.width / 6;
    double height = 3 * width / 4;
    return SliverToBoxAdapter(
      child: SizedBox(
        height: height,
        child: SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          child: Wrap(
            direction: Axis.horizontal,
            spacing: 24,
            children: [
              FilterSnapshot(name: "Order by", width: width, height: height),
              FilterSnapshot(name: "Actors", width: width, height: height),
              FilterSnapshot(name: "Director", width: width, height: height),
              FilterSnapshot(name: "Studio", width: width, height: height),
              FilterSnapshot(name: "Genres", width: width, height: height),
              FilterSnapshot(name: "Decade", width: width, height: height),
              FilterSnapshot(name: "someelse", width: width, height: height),
              FilterSnapshot(name: "Order by", width: width, height: height),
              FilterSnapshot(name: "Actors", width: width, height: height),
              FilterSnapshot(name: "Director", width: width, height: height),
              FilterSnapshot(name: "Studio", width: width, height: height),
              FilterSnapshot(name: "Genres", width: width, height: height),
              FilterSnapshot(name: "Decade", width: width, height: height),
              FilterSnapshot(name: "someelse", width: width, height: height),
              FilterSnapshot(name: "Order by", width: width, height: height),
              FilterSnapshot(name: "Actors", width: width, height: height),
              FilterSnapshot(name: "Director", width: width, height: height),
              FilterSnapshot(name: "Studio", width: width, height: height),
              FilterSnapshot(name: "Genresx", width: width, height: height),
              FilterSnapshot(name: "Decadxs", width: width, height: height),
              FilterSnapshot(name: "someelse", width: width, height: height),
            ],
          ),
        ),
      ),
    );
  }
}
