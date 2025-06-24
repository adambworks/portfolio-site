/* eslint-disable @typescript-eslint/no-unused-vars */
import { useState } from "react";
import type { Project } from "../structs/project";
import type { Chapter } from "../structs/chapter";
import { Link } from "react-router";

export default function HamburgerMenu(projects: Project[]) {
	//   const location = useLocation();
	const [open, setOpen] = useState(false);

	return (
		<div className="text-colorc px-4 py-3">
			<div className="flex items-center justify-between">
				<button
					className="text-colorc text-2x1 focus:outline-none"
					onClick={() => setOpen(!open)}
				>
					☰
				</button>
			</div>

			{open && (
				<div className="mt-2 flex flex-col gap-2 text-left">
					<Link
						//   key=""
						to="/"
						className="hover:underline"
						onClick={() => setOpen(false)}
					>
						Home
					</Link>
					<Link
						to="/projects"
						className="hover:underline"
						onClick={() => setOpen(false)}
					>
						Projects
					</Link>
					<div className="border-colorb/60 border-l pl-4 flex flex-col">
						{projects.map((project) => (
								<Link
									to={`/projects/${project.slug}`}
									className="hover:underline"
									onClick={() => setOpen(false)}
								>
									- {project.name}
								</Link>
						))}
					</div>
				</div>
			)}
		</div>
	);
}
