import { useEffect, useState } from "react";
import { Link, useParams } from "react-router";
import { fetchEntriesForChapter, updateEntry, deleteEntry, createEntry, uploadImage } from "../api/entries";
import type { Entry, NewEntry } from "../structs/entry";
import type { Chapter } from "../structs/chapter";
import { fetchChapter, fetchChapters } from "../api/chapters";
import HamburgerMenu from "../modules/global_buttons";
import type { Project } from "../structs/project";
import { fetchProjects } from "../api/projects";
import { useAuth } from "../context/AuthContext";
import ConfirmationModal from "../modules/ConfirmationModal";

function chapter_buttons(chapter: Chapter, slug: string, chapters: Chapter[]) {
  return (
    <div className="grid grid-cols-2 space-x-20">
      <div
        className={`rounded bg-colorb px-4 py-2 text-colorc transition hover:bg-colorb/70 shadow-sm ${
          chapter.index <= 1 || !chapters.some((c) => c.index === chapter.index - 1) ? "invisible" : ""
        }`}
      >
        <Link to={`/projects/${slug}/chapter/${chapter.index - 1}`}>
          <h2>
            Previous : {chapters.find((c) => c.index === chapter.index - 1)?.name}
          </h2>
        </Link>
      </div>

      <div
        className={`rounded bg-colorb px-4 py-2 text-colorc transition hover:bg-colorb/70 shadow-sm ${
          !chapters.some((c) => c.index === chapter.index + 1) ? "invisible" : ""
        }`}
      >
        <Link to={`/projects/${slug}/chapter/${chapter.index + 1}`}>
          <h2>
            Next : {chapters.find((c) => c.index === chapter.index + 1)?.name}
          </h2>
        </Link>
      </div>
    </div>
  );
}

export default function EntriesPage() {
  const { slug, chapterIndex } = useParams();
  const [entries, setEntries] = useState<Entry[]>([]);
  const [chapter, setChapter] = useState<Chapter | null>(null);
  const [loading, setLoading] = useState(true);
  const [chapters, setChapters] = useState<Chapter[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [editingEntry, setEditingEntry] = useState<Entry | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [entryToDelete, setEntryToDelete] = useState<number | null>(null);
  const { isAuthenticated } = useAuth();
  const API_URL = import.meta.env.VITE_API_URL;

  useEffect(() => {
    fetchProjects().then(setProjects).catch(console.error);
  }, []);

  useEffect(() => {
    if (slug && chapterIndex) {
      fetchChapter(slug, +chapterIndex).then(setChapter).catch(console.error);
    }
  }, [slug, chapterIndex]);

  useEffect(() => {
    if (chapter == null) return;
    fetchEntriesForChapter(chapter.id)
      .then(setEntries)
      .catch(console.error)
      .finally(() => setLoading(false));
  }, [chapter]);

  useEffect(() => {
    if (chapter == null) return;
    fetchChapters(chapter.project_id).then(setChapters).catch(console.error);
  }, [chapter]);

  const handleEdit = (entry: Entry) => {
    setEditingEntry({ ...entry });
  };

  const handleSave = async () => {
    if (editingEntry) {
      await updateEntry(editingEntry.id, editingEntry);
      setEntries(entries.map((e) => (e.id === editingEntry.id ? editingEntry : e)));
      setEditingEntry(null);
    }
  };

  const handleDeleteClick = (id: number) => {
    setEntryToDelete(id);
    setIsModalOpen(true);
  };

  const confirmDelete = async () => {
    if (entryToDelete !== null) {
      await deleteEntry(entryToDelete);
      setEntries(entries.filter((e) => e.id !== entryToDelete));
      setEntryToDelete(null);
    }
  };

  const handleCreate = async () => {
    if (chapter) {
      const newEntry: NewEntry = {
        chapter_id: chapter.id,
        text: "New Entry",
        date: new Date().toISOString().split("T")[0],
        image: "",
        index: entries.length > 0 ? Math.max(...entries.map((e) => e.index)) + 1 : 1,
      };
      const createdEntry = await createEntry(newEntry);
      setEntries([...entries, createdEntry]);
    }
  };

  if (loading) {
    return (
      <div className="p-4">
        {HamburgerMenu(projects)}
        Loading...
      </div>
    );
  }
  if (!chapter)
    return (
      <div className="p-4">
        {HamburgerMenu(projects)}
        Invalid Chapter
      </div>
    );
  if (!slug)
    return (
      <div className="p-4">
        {HamburgerMenu(projects)}
        Invalid Slug
      </div>
    );

  return (
    <div>
      <ConfirmationModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onConfirm={confirmDelete}
        message="Are you sure you want to delete this entry? This action cannot be undone."
      />
      {HamburgerMenu(projects)}
      <div className="mx-auto max-w-31/32 p-4">
        <h1 className="mb-4 text-2xl font-bold">{chapter?.name}</h1>
        {isAuthenticated && (
          <button onClick={handleCreate} className="my-4 rounded bg-blue-500 px-4 py-2 text-white">
            Create New Entry
          </button>
        )}
        {entries.length === 0 ? (
          <p className="text-gray-600">No entries available for this chapter.</p>
        ) : (
          <div className="grid grid-cols-1 gap-4">
            {entries.map((entry) => (
              <div key={entry.id} className="rounded-lg border-8 bg-colorc p-4">
                {editingEntry && editingEntry.id === entry.id ? (
                  <div>
                    <textarea
                      value={editingEntry.text ?? ""}
                      onChange={(e) => setEditingEntry({ ...editingEntry, text: e.target.value })}
                      className="mb-2 w-full rounded border p-2"
                    />
                    <input
                      type="date"
                      value={editingEntry.date ?? ""}
                      onChange={(e) => setEditingEntry({ ...editingEntry, date: e.target.value })}
                      className="mb-2 w-full rounded border p-2"
                    />
                    <input
                      type="file"
                      onChange={async (e) => {
                        if (e.target.files) {
                          const filename = await uploadImage(e.target.files[0]);
                          setEditingEntry({ ...editingEntry, image: filename });
                        }
                      }}
                      className="mb-2 w-full rounded border p-2"
                    />
                    <button onClick={handleSave} className="mr-2 rounded bg-green-500 px-4 py-2 text-white">
                      Save
                    </button>
                    <button onClick={() => setEditingEntry(null)} className="rounded bg-gray-500 px-4 py-2 text-white">
                      Cancel
                    </button>
                  </div>
                ) : (
                  <div>
                    <p className="text-2xl">{entry.text}</p>
                    {entry.date && <p className="text-colora">Date: {entry.date}</p>}
                    {entry.image && (
                      <img
                        src={`${API_URL}/images/${entry.image}`}
                        alt=""
                        className="mb-2 mt-2 h-auto w-full place-self-center object-contain"
                      />
                    )}
                    {isAuthenticated && (
                      <div className="mt-4">
                        <button
                          onClick={() => handleEdit(entry)}
                          className="mr-2 rounded bg-yellow-500 px-4 py-2 text-white"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleDeleteClick(entry.id)}
                          className="rounded bg-red-500 px-4 py-2 text-white"
                        >
                          Delete
                        </button>
                      </div>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
      {chapter_buttons(chapter, slug, chapters)}
    </div>
  );
}
