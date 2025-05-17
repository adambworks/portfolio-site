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
    <div className="p-4">
      <h1 className="text-xl font-bold mb-4">Projects</h1>
      <ul>
        {projects.map((project) => (
          <li key={project.id} className="mb-2">
            <Link to={`/projects/${project.slug}`} className="text-blue-500 hover:underline">
                <strong>{project.name}</strong> – {project.date_started}
            </Link>{" "}
            <p>{project.overview}</p>
          </li>
        ))}
      </ul>
    </div>
  );
}
