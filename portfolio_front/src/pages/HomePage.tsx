import { Link } from 'react-router';

export default function HomePage() {
  return (
    <div className="max-w-3xl mx-auto p-6 text-white">
      <h1 className="text-4xl font-bold mb-4">Welcome to My Portfolio</h1>

      <section className="mb-8">
        <h2 className="text-2xl font-semibold mb-2">About This Site</h2>
        <p>
          This is a full-stack portfolio site built using Rust (Actix Web) for the backend and React (Vite) for the frontend.
          It showcases a collection of projects, each with detailed chapters and entries that walk through the development process.
        </p>
      </section>

      <section className="mb-8">
        <h2 className="text-2xl font-semibold mb-2">About Me</h2>
        <p>
          I'm a developer who enjoys building projects from the ground up. I work with a mix of technologies, including Rust, TypeScript, Docker, and more.
          This site is both a technical showcase and a place where I document my work and ideas.
        </p>
      </section>

      <section className="mb-8">
        <Link
          to="/projects"
          className="inline-block bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition"
        >
          View Projects
        </Link>
      </section>

      <section className="mb-8">
        <h2 className="text-2xl font-semibold mb-2">Connect</h2>
        <a
          href="https://www.linkedin.com/in/your-linkedin-profile"
          target="_blank"
          rel="noopener noreferrer"
          className="text-blue-600 hover:underline"
        >
          LinkedIn Profile
        </a>
      </section>

      <section>
        <h2 className="text-2xl font-semibold mb-2">Contact</h2>
        <p>
          You can reach me at:{' '}
          <a
            href="mailto:your.email@example.com"
            className="text-blue-600 hover:underline"
          >
            your.email@example.com
          </a>
        </p>
      </section>
    </div>
  );
}
