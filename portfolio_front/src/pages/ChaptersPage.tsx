import { Link, useParams } from "react-router";
import { useEffect, useState } from "react";
import { fetchProjectBySlug, fetchProjects } from "../api/projects";
import type { Project } from "../structs/project";
import type { Chapter, NewChapter } from "../structs/chapter";
import { fetchChapters, updateChapter, deleteChapter, createChapter } from "../api/chapters";
import HamburgerMenu from "../modules/global_buttons";
import { useAuth } from "../context/AuthContext";
import ConfirmationModal from "../modules/ConfirmationModal";

function chapter_descriptor(project: Project, chapter_index: number) {
  if (project.chapter_descriptor != null) {
    return (
      <h2>
        {project.chapter_descriptor}: {chapter_index}
      </h2>
    );
  } else {
    return;
  }
}

export default function ChaptersPage() {
  const { slug } = useParams<{ slug: string }>();
  const [project, setProject] = useState<Project | null>(null);
  const [chapters, setChapters] = useState<Chapter[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [editingChapter, setEditingChapter] = useState<Chapter | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [chapterToDelete, setChapterToDelete] = useState<number | null>(null);
  const { isAuthenticated } = useAuth();
  const API_URL = import.meta.env.VITE_API_URL;

  useEffect(() => {
    fetchProjects().then(setProjects).catch(console.error);
  }, []);

  useEffect(() => {
    if (slug) {
      fetchProjectBySlug(slug).then(setProject).catch(console.error);
    }
  }, [slug]);

  useEffect(() => {
    if (!project) return;
    fetchChapters(project.id).then(setChapters).catch(console.error);
  }, [project]);

  const handleEdit = (chapter: Chapter) => {
    setEditingChapter({ ...chapter });
  };

  const handleSave = async () => {
    if (editingChapter) {
      await updateChapter(editingChapter.id, editingChapter);
      setChapters(chapters.map((c) => (c.id === editingChapter.id ? editingChapter : c)));
      setEditingChapter(null);
    }
  };

  const handleDeleteClick = (id: number) => {
    setChapterToDelete(id);
    setIsModalOpen(true);
  };

  const confirmDelete = async () => {
    if (chapterToDelete !== null) {
      await deleteChapter(chapterToDelete);
      setChapters(chapters.filter((c) => c.id !== chapterToDelete));
      setChapterToDelete(null);
    }
  };

  const handleCreate = async () => {
    if (project) {
      const newChapter: NewChapter = {
        project_id: project.id,
        name: "New Chapter",
        date_started: new Date().toISOString().split("T")[0],
        index: chapters.length > 0 ? Math.max(...chapters.map((c) => c.index)) + 1 : 1,
      };
      const createdChapter = await createChapter(newChapter);
      setChapters([...chapters, createdChapter]);
    }
  };

  if (!project) return <div className="p-4">{HamburgerMenu(projects)} Invalid project</div>;

  return (
    <div className="p-4">
      <ConfirmationModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onConfirm={confirmDelete}
        message="Are you sure you want to delete this chapter? This action cannot be undone."
      />
      {HamburgerMenu(projects)}

      {project.image && (
        <img
          src={`${API_URL}/images/${project.image}`}
          alt={project.name}
          className="mb-2 h-40 w-full rounded object-cover"
        />
      )}
      <h1 className="mb-2 text-2xl font-bold">{project.name}</h1>
      <p className="mb-4 text-colore">{project.date_started}</p>

      <h2 className="text-xl">Overview:</h2>
      <p className="text-colore">{project.overview}</p>

      {isAuthenticated && (
        <button onClick={handleCreate} className="my-4 rounded bg-blue-500 px-4 py-2 text-white">
          Create New Chapter
        </button>
      )}

      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3">
        {chapters.map((chapter) => (
          <div key={chapter.id} className="rounded-lg border-8 bg-colorc p-2">
            {editingChapter && editingChapter.id === chapter.id ? (
              <div>
                <input
                  type="text"
                  value={editingChapter.name}
                  onChange={(e) => setEditingChapter({ ...editingChapter, name: e.target.value })}
                  className="mb-2 w-full rounded border p-2"
                />
                <button onClick={handleSave} className="mr-2 rounded bg-green-500 px-4 py-2 text-white">
                  Save
                </button>
                <button onClick={() => setEditingChapter(null)} className="rounded bg-gray-500 px-4 py-2 text-white">
                  Cancel
                </button>
              </div>
            ) : (
              <div>
                <Link to={`/projects/${project.slug}/chapter/${chapter.index}`}>
                  {chapter_descriptor(project, chapter.index)}
                  <h2 className="mb-1 text-lg font-semibold">{chapter.name}</h2>
                </Link>
                {isAuthenticated && (
                  <div className="mt-4">
                    <button onClick={() => handleEdit(chapter)} className="mr-2 rounded bg-yellow-500 px-4 py-2 text-white">
                      Edit
                    </button>
                    <button onClick={() => handleDeleteClick(chapter.id)} className="rounded bg-red-500 px-4 py-2 text-white">
                      Delete
                    </button>
                  </div>
                )}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
