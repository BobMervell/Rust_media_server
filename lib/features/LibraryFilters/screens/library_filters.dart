import 'package:fluster_media_center/features/LibraryFilters/widgets/item_snapshot.dart';
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
              ItemSnapshot(name: "Order by", width: width, height: height),
              ItemSnapshot(name: "Actors", width: width, height: height),
              ItemSnapshot(name: "Director", width: width, height: height),
              ItemSnapshot(name: "Studio", width: width, height: height),
              ItemSnapshot(name: "Genres", width: width, height: height),
              ItemSnapshot(name: "Decade", width: width, height: height),
              ItemSnapshot(name: "someelse", width: width, height: height),
              ItemSnapshot(name: "Order by", width: width, height: height),
              ItemSnapshot(name: "Actors", width: width, height: height),
              ItemSnapshot(name: "Director", width: width, height: height),
              ItemSnapshot(name: "Studio", width: width, height: height),
              ItemSnapshot(name: "Genres", width: width, height: height),
              ItemSnapshot(name: "Decade", width: width, height: height),
              ItemSnapshot(name: "someelse", width: width, height: height),
              ItemSnapshot(name: "Order by", width: width, height: height),
              ItemSnapshot(name: "Actors", width: width, height: height),
              ItemSnapshot(name: "Director", width: width, height: height),
              ItemSnapshot(name: "Studio", width: width, height: height),
              ItemSnapshot(name: "Genresx", width: width, height: height),
              ItemSnapshot(name: "Decadxs", width: width, height: height),
              ItemSnapshot(name: "someelse", width: width, height: height),
            ],
          ),
        ),
      ),
    );
  }
}
