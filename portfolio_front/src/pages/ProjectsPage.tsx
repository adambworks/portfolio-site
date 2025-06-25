import { useEffect, useState } from "react";
import { fetchProjects } from "../api/projects";
import type { Project } from "../structs/project";
import { Link } from "react-router";
import HamburgerMenu from "../modules/global_buttons";

export default function ProjectsPage() {
	const [projects, setProjects] = useState<Project[]>([]);
  const API_URL = import.meta.env.VITE_API_URL;

	useEffect(() => {
		fetchProjects().then(setProjects).catch(console.error);
	}, []);

	return (
		<div className="mx-auto max-w-6xl p-4">
      {HamburgerMenu(projects)}
			<h1 className="mb-6 text-2xl font-bold">Projects</h1>

			<div className="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3">
				{projects.map((project) => (
					<Link
						key={project.slug}
						to={`/projects/${project.slug}`}
						className="block rounded-lg border-8 bg-colorc p-4 shadow transition hover:shadow-md"
					>
						{project.image !== null && project.image != "" && (
							<img
								src={`${API_URL}/images/${project.image}`}
								alt=""
								className="mb-2 h-40 w-full rounded object-cover"
							/>
						)}
						<h2 className="mb-1 text-lg font-semibold">{project.name}</h2>
						<p className="mb-2 text-xs text-colora font-semibold">{project.date_started}</p>
						<p className="text-sm">{project.overview}</p>
					</Link>
				))}
			</div>
		</div>
	);
}
