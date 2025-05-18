import { useEffect, useState } from "react";
import { useParams } from "react-router";
import { fetchEntriesForChapter } from "../api/entries";
import type { Entry } from "../structs/entry";
import type { Chapter } from "../structs/chapter";
import { fetchChapter } from "../api/chapters";

export default function ChapterPage() {
  const { slug, chapterIndex } = useParams();
  const [entries, setEntries] = useState<Entry[]>([]);
  const [chapter, setChapter] = useState<Chapter|null>(null)
  const [loading, setLoading] = useState(true);

  console.log(slug,chapterIndex);
  useEffect(() =>{ 
    if (slug && chapterIndex){
        fetchChapter(slug,+chapterIndex).then(setChapter).catch(console.error);
    }

  },[slug,chapterIndex])

  useEffect(() => {
     if(chapter==null) return;
      fetchEntriesForChapter(chapter.id)
        .then(setEntries)
        .catch(console.error)
        .finally(() => setLoading(false));
  
  }, [chapter]);

  if (loading) {
    return <div className="p-4">Loading...</div>;
  }

  return (
    <div className="p-4 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold mb-4">Entries for Chapter {chapterIndex}</h1>
      {entries.length === 0 ? (
        <p className="text-gray-600">No entries available for this chapter.</p>
      ) : (
        <div className="space-y-4">
          {entries.map((entry) => (
            <div
              key={entry.id}
              className="p-4 border border-gray-200 rounded shadow-sm bg-white"
            >
              {entry.image && (
                <img
                  src={`http://localhost:8080/api/images/${entry.image}`}
                  alt=""
                  className="w-full h-60 object-cover rounded mb-2"
                />
              )}
              <p className="text-gray-700">{entry.text}</p>
              <p className="text-gray-700">Date: {entry.date}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
