export default function Unauthorized() {
  return (
    <div className="min-h-screen flex items-center justify-center">
      <div className="card-gaming max-w-md w-full mx-4 text-center">
        <h1 className="text-3xl font-heading text-destructive mb-4">Access Denied</h1>
        <p className="text-muted-foreground mb-6">
          You do not have the required admin role to access this portal.
        </p>
        <a
          href="/"
          className="inline-block px-6 py-3 bg-primary text-white rounded-lg hover:bg-primary/90 transition"
        >
          Return to Home
        </a>
      </div>
    </div>
  );
}
