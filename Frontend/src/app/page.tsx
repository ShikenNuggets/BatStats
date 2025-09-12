"use client"

import TimeTable from "@/components/TimeTable";
//import Image from "next/image";
//import styles from "./page.module.css";
import { useEffect, useState } from "react";

interface BatStatsData {
  world_records: [string, number][];
  runner_times: [string, number][];
  runner_ranks: [string, number][];
  any_times: [string, number][];
  glitchless_times: [string, number][];
  hundo_times: [string, number][];
  asylum_mastery: [string, number][];
  city_mastery: [string, number][];
  origins_mastery: [string, number][];
  knight_mastery: [string, number][];
  overall_mastery: [string, number][];
}

type DataKey = keyof BatStatsData;

const GIST_RAW_URL = "https://gist.githubusercontent.com/ShikenNuggets/3adaa36be92dfb82f43b951b91387c1a/raw/924f925045bb0f2ff5229a68dd543d990c0bc6e9/BatStats.json";

export default function Home() {
  const [data, setData] = useState<BatStatsData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setLoading] = useState(true);
  const [selectedKey, setSelectedKey] = useState<DataKey>('any_times');

  useEffect(() => {
    const fetchData = async() => {
      try{
        const result = await fetch(GIST_RAW_URL);
        if (!result.ok){
          throw new Error("Failed to fetch: ${res.status}");
        }

        const json = await result.json();
        setData(json as BatStatsData);
      }catch(err: unknown){
        if (err instanceof Error){
          setError(err.message);
        }else{
          setError("Unknown error");
        }
        
      }finally{
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if(isLoading){
    return <div>Loading...</div>;
  }else if(error){
    return <div style={{color: 'red' }}>Error: {error} - please try again later</div>;
  }else if(!data){
    return <div>No data available - please try again later.</div>;
  }
  
  const buttons: { key: DataKey; buttonLabel: string; dataLabel: string }[] = [
    { key: 'any_times', buttonLabel: 'Any%', dataLabel: 'Overall Any% Times' },
    { key: 'glitchless_times', buttonLabel: 'Glitchless', dataLabel: 'Overall Glitchless Times' },
    { key: 'hundo_times', buttonLabel: '100%', dataLabel: 'Overall 100% Times' },
    { key: 'asylum_mastery', buttonLabel: 'Asylum', dataLabel: 'Asylum Mastery' },
    { key: 'city_mastery', buttonLabel: 'City', dataLabel: 'City Mastery' },
    { key: 'origins_mastery', buttonLabel: 'Origins', dataLabel: 'Origins Mastery' },
    { key: 'knight_mastery', buttonLabel: 'Knight', dataLabel: 'Knight Mastery' },
    { key: 'overall_mastery', buttonLabel: 'Overall', dataLabel: 'Overall Mastery' },
    { key: 'world_records', buttonLabel: 'WRs', dataLabel: '# of World Records' },
    { key: 'runner_times', buttonLabel: 'Times', dataLabel: 'Overall Leaderboard Times' },
    { key: 'runner_ranks', buttonLabel: 'Ranks', dataLabel: 'Overall Leaderboard Ranks' },
  ];

  return (
    <div>
      <h1 style={{ paddingTop: '10px', textAlign: 'center' }}>Batman Arkham Speedrunning Stats</h1>

      {/* Buttons */}
      <div style={{ display: 'flex', flexWrap: 'wrap', justifyContent: 'center', gap: '10px', margin: '20px 0' }}>
        {buttons.map((btn) => (
          <button
            key={btn.key}
            onClick={() => setSelectedKey(btn.key)}
            style={{
              padding: '10px 15px',
              cursor: 'pointer',
              backgroundColor: selectedKey === btn.key ? '#444' : '#eee',
              color: selectedKey === btn.key ? 'white' : 'black',
              border: '1px solid #ccc',
              borderRadius: '5px',
              transition: 'background-color 0.2s',
            }}
          >
            {btn.buttonLabel}
          </button>
        ))}
      </div>

      <TimeTable
        data={data[selectedKey]}
        title={buttons.find((b) => b.key === selectedKey)?.dataLabel || 'Data'}
        tableKey="Runner"
        tableValue="Time"
      />
    </div>
  );
}
