"use client";

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { Plus, ArrowLeft, Trash2, X } from "lucide-react";
import Link from "next/link";
import { useState } from "react";

interface CatalogItem {
  id: number;
  name: string;
}

interface RaceGenderAllowed {
  race_id: number;
  gender_id: number;
}

interface RaceGenderSkinColorAllowed {
  race_id: number;
  gender_id: number;
  skin_color_id: number;
}

interface RaceGenderClassAllowed {
  race_id: number;
  gender_id: number;
  class_id: number;
}

interface CombinationData {
  race_id: number;
  gender_id: number;
  skin_colors: number[];
  classes: number[];
}

export default function CombinationsPage() {
  const [selectedRace, setSelectedRace] = useState<number | null>(null);
  const [selectedGender, setSelectedGender] = useState<number | null>(null);
  const [showAddForm, setShowAddForm] = useState(false);

  // Fetch catalog data
  const { data: races } = useQuery<CatalogItem[]>({ 
    queryKey: ['races'], 
    queryFn: async () => (await fetch('/api/catalog/races')).json() 
  });
  const { data: genders } = useQuery<CatalogItem[]>({ 
    queryKey: ['genders'], 
    queryFn: async () => (await fetch('/api/catalog/genders')).json() 
  });
  const { data: skinColors } = useQuery<CatalogItem[]>({ 
    queryKey: ['skincolors'], 
    queryFn: async () => (await fetch('/api/catalog/skincolors')).json() 
  });
  const { data: classes } = useQuery<CatalogItem[]>({ 
    queryKey: ['classes'], 
    queryFn: async () => (await fetch('/api/catalog/classes')).json() 
  });

  // Fetch combinations
  const { data: raceGenderCombos, isLoading: loadingRaceGender } = useQuery<RaceGenderAllowed[]>({
    queryKey: ['combinations', 'race-gender'],
    queryFn: async () => {
      const res = await fetch('/api/catalog/combinations/race-gender');
      if (!res.ok) throw new Error('Failed to fetch race-gender combinations');
      return res.json();
    },
  });

  const { data: raceGenderSkinColorCombos } = useQuery<RaceGenderSkinColorAllowed[]>({
    queryKey: ['combinations', 'race-gender-skin-color'],
    queryFn: async () => {
      const res = await fetch('/api/catalog/combinations/race-gender-skin-color');
      if (!res.ok) throw new Error('Failed to fetch race-gender-skin-color combinations');
      return res.json();
    },
  });

  const { data: raceGenderClassCombos } = useQuery<RaceGenderClassAllowed[]>({
    queryKey: ['combinations', 'race-gender-class'],
    queryFn: async () => {
      const res = await fetch('/api/catalog/combinations/race-gender-class');
      if (!res.ok) throw new Error('Failed to fetch race-gender-class combinations');
      return res.json();
    },
  });

  const getRaceName = (id: number) => races?.find(r => r.id === id)?.name || `Race ${id}`;
  const getGenderName = (id: number) => genders?.find(g => g.id === id)?.name || `Gender ${id}`;
  const getSkinColorName = (id: number) => skinColors?.find(s => s.id === id)?.name || `Skin Color ${id}`;
  const getClassName = (id: number) => classes?.find(c => c.id === id)?.name || `Class ${id}`;

  // Build combined data structure
  const combinedData: CombinationData[] = (raceGenderCombos || []).map(rg => {
    const skinColorIds = (raceGenderSkinColorCombos || [])
      .filter(rgsc => rgsc.race_id === rg.race_id && rgsc.gender_id === rg.gender_id)
      .map(rgsc => rgsc.skin_color_id);
    
    const classIds = (raceGenderClassCombos || [])
      .filter(rgc => rgc.race_id === rg.race_id && rgc.gender_id === rg.gender_id)
      .map(rgc => rgc.class_id);

    return {
      race_id: rg.race_id,
      gender_id: rg.gender_id,
      skin_colors: skinColorIds,
      classes: classIds,
    };
  });

  const handleAddCombination = () => {
    setShowAddForm(true);
  };

  return (
    <div>
      <Link href="/admin/catalog" className="inline-flex items-center gap-2 text-muted-foreground hover:text-foreground mb-4">
        <ArrowLeft size={16} />
        Back to Catalog
      </Link>

      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-3xl font-heading">Allowed Combinations</h1>
          <p className="text-sm text-muted-foreground mt-1">
            Configure which race-gender pairs are allowed and their valid skin colors and classes
          </p>
        </div>
        <button
          onClick={handleAddCombination}
          className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 flex items-center gap-2"
        >
          <Plus size={16} />
          Add Combination
        </button>
      </div>

      {/* Add Form Modal */}
      {showAddForm && (
        <div className="card-gaming mb-6 p-6 border-2 border-primary/50">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-heading">Add New Combination</h3>
            <button onClick={() => setShowAddForm(false)} className="text-muted-foreground hover:text-foreground">
              <X size={20} />
            </button>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium mb-2">Race</label>
              <select
                value={selectedRace || ''}
                onChange={(e) => setSelectedRace(Number(e.target.value))}
                className="w-full px-3 py-2 bg-muted rounded-lg border border-border focus:border-primary outline-none"
              >
                <option value="">Select race...</option>
                {races?.map(race => (
                  <option key={race.id} value={race.id}>{race.name}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium mb-2">Gender</label>
              <select
                value={selectedGender || ''}
                onChange={(e) => setSelectedGender(Number(e.target.value))}
                className="w-full px-3 py-2 bg-muted rounded-lg border border-border focus:border-primary outline-none"
              >
                <option value="">Select gender...</option>
                {genders?.map(gender => (
                  <option key={gender.id} value={gender.id}>{gender.name}</option>
                ))}
              </select>
            </div>
          </div>
          <div className="flex gap-2 mt-4">
            <button
              disabled={!selectedRace || !selectedGender}
              className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Add Base Combination
            </button>
            <button
              onClick={() => {
                setShowAddForm(false);
                setSelectedRace(null);
                setSelectedGender(null);
              }}
              className="px-4 py-2 bg-muted text-foreground rounded-lg hover:bg-muted/80"
            >
              Cancel
            </button>
          </div>
        </div>
      )}

      {/* Combinations Table */}
      <div className="card-gaming overflow-hidden">
        {loadingRaceGender ? (
          <p className="text-muted-foreground p-6">Loading...</p>
        ) : combinedData.length === 0 ? (
          <p className="text-muted-foreground p-6">No combinations found. Add your first combination to get started.</p>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-border bg-muted/30">
                  <th className="text-left p-4 font-heading">Race</th>
                  <th className="text-left p-4 font-heading">Gender</th>
                  <th className="text-left p-4 font-heading">Allowed Skin Colors</th>
                  <th className="text-left p-4 font-heading">Allowed Classes</th>
                  <th className="text-right p-4 font-heading">Actions</th>
                </tr>
              </thead>
              <tbody>
                {combinedData.map((combo, idx) => (
                  <tr key={idx} className="border-b border-border hover:bg-muted/20">
                    <td className="p-4">
                      <span className="font-medium">{getRaceName(combo.race_id)}</span>
                    </td>
                    <td className="p-4">
                      <span className="font-medium">{getGenderName(combo.gender_id)}</span>
                    </td>
                    <td className="p-4">
                      <div className="flex flex-wrap gap-1">
                        {combo.skin_colors.length > 0 ? (
                          combo.skin_colors.map(scId => (
                            <span key={scId} className="inline-flex items-center gap-1 px-2 py-1 bg-orange-500/20 text-orange-300 rounded text-sm">
                              {getSkinColorName(scId)}
                              <button className="hover:text-orange-100">
                                <X size={12} />
                              </button>
                            </span>
                          ))
                        ) : (
                          <span className="text-muted-foreground text-sm">None</span>
                        )}
                        <button className="px-2 py-1 bg-muted hover:bg-muted/70 rounded text-xs flex items-center gap-1">
                          <Plus size={12} />
                          Add
                        </button>
                      </div>
                    </td>
                    <td className="p-4">
                      <div className="flex flex-wrap gap-1">
                        {combo.classes.length > 0 ? (
                          combo.classes.map(classId => (
                            <span key={classId} className="inline-flex items-center gap-1 px-2 py-1 bg-red-500/20 text-red-300 rounded text-sm">
                              {getClassName(classId)}
                              <button className="hover:text-red-100">
                                <X size={12} />
                              </button>
                            </span>
                          ))
                        ) : (
                          <span className="text-muted-foreground text-sm">None</span>
                        )}
                        <button className="px-2 py-1 bg-muted hover:bg-muted/70 rounded text-xs flex items-center gap-1">
                          <Plus size={12} />
                          Add
                        </button>
                      </div>
                    </td>
                    <td className="p-4 text-right">
                      <button className="px-3 py-1 bg-destructive/20 text-destructive hover:bg-destructive/30 rounded text-sm inline-flex items-center gap-1">
                        <Trash2 size={14} />
                        Delete
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Summary Stats */}
      <div className="grid grid-cols-3 gap-4 mt-6">
        <div className="card-gaming p-4">
          <div className="text-2xl font-heading text-blue-400">{combinedData.length}</div>
          <div className="text-sm text-muted-foreground">Race-Gender Pairs</div>
        </div>
        <div className="card-gaming p-4">
          <div className="text-2xl font-heading text-orange-400">
            {(raceGenderSkinColorCombos || []).length}
          </div>
          <div className="text-sm text-muted-foreground">Skin Color Options</div>
        </div>
        <div className="card-gaming p-4">
          <div className="text-2xl font-heading text-red-400">
            {(raceGenderClassCombos || []).length}
          </div>
          <div className="text-sm text-muted-foreground">Class Options</div>
        </div>
      </div>
    </div>
  );
}
