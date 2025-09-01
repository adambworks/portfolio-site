import { useState } from 'react';
import { useNavigate } from 'react-router';
import { login as apiLogin } from '../api/auth';
import { useAuth } from '../context/AuthContext';

function LoginPage() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const { login } = useAuth();
  const navigate = useNavigate();

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setError(null);
    try {
      const data = await apiLogin(username, password);
      login(data.token);
      navigate('/');
    } catch (err) {
      setError('Failed to login. Please check your credentials.');
    }
  };

  return (
    <div className="mx-auto max-w-md p-24">
      <h1 className="mb-4 text-center text-4xl font-bold">Login</h1>
      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label htmlFor="username" className="mb-2 block text-colore">
            Username
          </label>
          <input
            type="text"
            id="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
            className="w-full rounded border bg-gray-800 px-3 py-2 text-white"
          />
        </div>
        <div className="mb-6">
          <label htmlFor="password" className="mb-2 block text-colore">
            Password
          </label>
          <input
            type="password"
            id="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
            className="w-full rounded border bg-gray-800 px-3 py-2 text-white"
          />
        </div>
        <button
          type="submit"
          className="w-full rounded bg-colorb px-4 py-2 text-colorc transition hover:bg-colorb/70"
        >
          Login
        </button>
      </form>
      {error && <p className="mt-4 text-center text-red-500">{error}</p>}
    </div>
  );
}

export default LoginPage;