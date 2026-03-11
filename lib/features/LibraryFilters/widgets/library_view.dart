import 'package:fluster_media_center/features/LibraryFilters/widgets/media_snapshot.dart';
import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:fluster_media_center/src/rust/movie_data/movie_data.dart';
import 'package:flutter/material.dart';

class LibraryView extends StatefulWidget {
  const LibraryView({super.key});

  @override
  State<LibraryView> createState() => _LibraryViewState();
}

class _LibraryViewState extends State<LibraryView> {
  List<MovieSnapshot> medias = [];
  String? errorMessage;

  @override
  void initState() {
    super.initState();
    loadMedias();
  }

  void loadMedias() async {
    try {
      final result = await getMediaSnapshots(mediaType: "movie");
      setState(() {
        medias = result;
      });
    } catch (e) {
      print(e.toString());
    }
  }

  @override
  Widget build(BuildContext context) {
    double width = MediaQuery.of(context).size.width / 6;

    return SliverGrid.builder(
      gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
        maxCrossAxisExtent: width,
        mainAxisSpacing: 64,
        crossAxisSpacing: 24,
        childAspectRatio: 2 / 3,
      ),
      itemCount: medias.length,
      itemBuilder: (BuildContext context, int index) {
        final media = medias[index];
        return MediaSnapshot(media: media); // utilisation des vraies données
      },
    );
  }
}
