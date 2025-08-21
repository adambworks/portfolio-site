import { createRoot } from "react-dom/client";
import "./index.css";
import { BrowserRouter, Routes, Route } from "react-router";
import ProjectsPage from "./pages/ProjectsPage.tsx";
import ChapterPagesPage from "./pages/ChaptersPage.tsx";
import EntriesPage from "./pages/EntriesPage.tsx";
import HomePage from "./pages/HomePage.tsx";
import LoginPage from "./pages/LoginPage.tsx";
import { AuthProvider } from "./context/AuthContext.tsx";

createRoot(document.getElementById("root")!).render(
	<div className="bg-colora min-h-screen">
		<div className="text-colorb mx-auto max-w-screen-lg px-4 text-center">
			<AuthProvider>
				<BrowserRouter>
					<Routes>
						<Route path="/" element={<HomePage />} />
						<Route path="/projects" element={<ProjectsPage />} />
						<Route path="/login" element={<LoginPage />} />
						<Route path="/projects/:slug" element={<ChapterPagesPage />} />
						<Route path="/projects/:slug/chapter/:chapterIndex" element={<EntriesPage />}
						/>
					</Routes>
				</BrowserRouter>
			</AuthProvider>
		</div>
	</div>,
);
