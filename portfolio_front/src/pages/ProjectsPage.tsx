import { useEffect, useState } from "react";
import { fetchProjects } from "../api/projects";
import type { Project } from "../structs/project";
import { Link } from "react-router";

export default function ProjectsPage() {
  const [projects, setProjects] = useState<Project[]>([]);

  useEffect(() => {
    fetchProjects().then(setProjects).catch(console.error);
  }, []);

  return (
    <div className="p-4 max-w-6xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Projects</h1>

      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
        {projects.map((project) => (
          <Link
            key={project.slug}
            to={`/projects/${project.slug}`}
            className="block p-4 rounded-lg shadow hover:shadow-md transition bg-white border border-gray-200"
          >
            {project.image !== null && project.image!= "" &&(
             <img src={`http://localhost:8080/api/images/${project.image}`} alt="" className="mb-2 w-full h-40 object-cover rounded" /> 
            )}
            <h2 className="text-lg font-semibold mb-1">{project.name}</h2>
            <p className="text-sm text-gray-500 mb-2">{project.date_started}</p>
            <p className="text-gray-700 text-sm">{project.overview}</p>
          </Link>
        ))}
      </div>
    </div>
  );
}
