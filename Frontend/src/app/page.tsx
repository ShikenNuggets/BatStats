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

const GIST_RAW_URL = "https://gist.githubusercontent.com/ShikenNuggets/3adaa36be92dfb82f43b951b91387c1a/raw/924f925045bb0f2ff5229a68dd543d990c0bc6e9/BatStats.json";

export default function Home() {
  const [data, setData] = useState<BatStatsData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setLoading] = useState(true);

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

  return (
    <div>
      <h1 style={{ paddingTop: '10px', textAlign: 'center' }}>Batman Arkham Speedrunning Stats</h1>
      <TimeTable data={data.any_times} title="Overall Any% Times" tableKey="Runner" tableValue="Time" />
    </div>
  );
}
