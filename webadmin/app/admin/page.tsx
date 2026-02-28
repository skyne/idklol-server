export default function AdminDashboard() {
  return (
    <div>
      <h1 className="text-3xl font-heading mb-6">Dashboard</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="card-gaming">
          <h3 className="text-muted-foreground text-sm mb-2">Total Characters</h3>
          <p className="text-4xl font-heading text-primary">--</p>
        </div>
        
        <div className="card-gaming">
          <h3 className="text-muted-foreground text-sm mb-2">Catalog Items</h3>
          <p className="text-4xl font-heading text-secondary">--</p>
        </div>
        
        <div className="card-gaming">
          <h3 className="text-muted-foreground text-sm mb-2">Active Users</h3>
          <p className="text-4xl font-heading text-green-500">--</p>
        </div>
      </div>

      <div className="mt-8 card-gaming">
        <h2 className="text-xl font-heading mb-4">Welcome</h2>
        <p className="text-muted-foreground">
          Use the navigation menu to manage game catalog items and characters.
        </p>
      </div>
    </div>
  );
}
