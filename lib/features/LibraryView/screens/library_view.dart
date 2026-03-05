import 'package:fluster_media_center/features/LibraryView/widgets/item_snapshot.dart';
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
      final result = await getMedia(mediaType: "movie");
      setState(() {
        medias = result;
      });
    } catch (e) {
      print(e.toString());
    }
  }

  @override
  Widget build(BuildContext context) {
    return SliverGrid.builder(
      gridDelegate: const SliverGridDelegateWithMaxCrossAxisExtent(
        maxCrossAxisExtent: 200,
        mainAxisSpacing: 100,
        crossAxisSpacing: 20,
        childAspectRatio: 1,
      ),
      itemCount: medias.length,
      itemBuilder: (BuildContext context, int index) {
        final media = medias[index];
        return ItemSnapshot(
          name: media.title,
        ); // utilisation des vraies données
      },
    );
  }
}
