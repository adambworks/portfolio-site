import { useEffect, useState } from "react";
import { fetchProjects, updateProject, deleteProject, createProject } from "../api/projects";
import type { Project, NewProject } from "../structs/project";
import { Link } from "react-router";
import HamburgerMenu from "../modules/global_buttons";
import { useAuth } from "../context/AuthContext";

export default function ProjectsPage() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [editingProject, setEditingProject] = useState<Project | null>(null);
  const { isAuthenticated } = useAuth();
  const API_URL = import.meta.env.VITE_API_URL;

  useEffect(() => {
    fetchProjects().then(setProjects).catch(console.error);
  }, []);

  const handleEdit = (project: Project) => {
    setEditingProject({ ...project });
  };

  const handleSave = async () => {
    if (editingProject) {
      await updateProject(editingProject.id, editingProject);
      setProjects(projects.map((p) => (p.id === editingProject.id ? editingProject : p)));
      setEditingProject(null);
    }
  };

  const handleDelete = async (id: number) => {
    await deleteProject(id);
    setProjects(projects.filter((p) => p.id !== id));
  };

  const handleCreate = async () => {
    const name = "New Project";
    const slug = name.toLowerCase().replace(/\s+/g, "_");
    const newProject: NewProject = {
      name,
      slug,
      date_started: new Date().toISOString().split("T")[0],
      overview: "",
      image: "",
      chapter_descriptor: "Chapter",
    };
    const createdProject = await createProject(newProject);
    setProjects([...projects, createdProject]);
  };

  return (
    <div className="mx-auto max-w-6xl p-4">
      {HamburgerMenu(projects)}
      <h1 className="mb-6 text-2xl font-bold">Projects</h1>
      {isAuthenticated && (
        <button onClick={handleCreate} className="mb-4 rounded bg-blue-500 px-4 py-2 text-white">
          Create New Project
        </button>
      )}
      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3">
        {projects.map((project) => (
          <div key={project.id} className="rounded-lg border-8 bg-colorc p-4 shadow">
            {editingProject && editingProject.id === project.id ? (
              <div>
                <input
                  type="text"
                  value={editingProject.name}
                  onChange={(e) => setEditingProject({ ...editingProject, name: e.target.value })}
                  className="mb-2 w-full rounded border p-2"
                />
                <input
                  type="date"
                  value={editingProject.date_started}
                  onChange={(e) => setEditingProject({ ...editingProject, date_started: e.target.value })}
                  className="mb-2 w-full rounded border p-2"
                />
                <textarea
                  value={editingProject.overview ?? ""}
                  onChange={(e) => setEditingProject({ ...editingProject, overview: e.target.value })}
                  className="mb-2 w-full rounded border p-2"
                />
                <button onClick={handleSave} className="mr-2 rounded bg-green-500 px-4 py-2 text-white">
                  Save
                </button>
                <button onClick={() => setEditingProject(null)} className="rounded bg-gray-500 px-4 py-2 text-white">
                  Cancel
                </button>
              </div>
            ) : (
              <div>
                <Link to={`/projects/${project.slug}`}>
                  {project.image && (
                    <img
                      src={`${API_URL}/images/${project.image}`}
                      alt={project.name}
                      className="mb-2 h-40 w-full rounded object-cover"
                    />
                  )}
                  <h2 className="mb-1 text-lg font-semibold">{project.name}</h2>
                  <p className="mb-2 text-xs font-semibold text-colora">{project.date_started}</p>
                  <p className="text-sm">{project.overview}</p>
                </Link>
                {isAuthenticated && (
                  <div className="mt-4">
                    <button onClick={() => handleEdit(project)} className="mr-2 rounded bg-yellow-500 px-4 py-2 text-white">
                      Edit
                    </button>
                    <button onClick={() => handleDelete(project.id)} className="rounded bg-red-500 px-4 py-2 text-white">
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
