"use client"

import TimeTable, { ValueType } from "@/components/TimeTable";
//import Image from "next/image";
//import styles from "./page.module.css";
import { useEffect, useState } from "react";

type DataPair = {
  rank: number;
  player: string;
  value: number;
};

interface BatStatsData {
  world_records: DataPair[];
  runner_times: DataPair[];
  runner_ranks: DataPair[];
  any_times: DataPair[];
  glitchless_times: DataPair[];
  hundo_times: DataPair[];
  asylum_mastery: DataPair[];
  city_mastery: DataPair[];
  origins_mastery: DataPair[];
  knight_mastery: DataPair[];
  overall_mastery: DataPair[];
}

type DataKey = keyof BatStatsData;

interface DatasetConfig {
  key: DataKey;
  buttonLabel: string;
  tableTitle: string;
  valueType: ValueType;
}

const GIST_RAW_URL = "https://gist.githubusercontent.com/ShikenNuggets/3adaa36be92dfb82f43b951b91387c1a/raw/924f925045bb0f2ff5229a68dd543d990c0bc6e9/BatStats.json";

export default function Home() {
  const [data, setData] = useState<BatStatsData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setLoading] = useState(true);
  const [selectedKey, setSelectedKey] = useState<DataKey>('any_times');

  const datasetConfigs: DatasetConfig[] = [
    { key: 'any_times', buttonLabel: 'Any%', tableTitle: 'Overall Any% Times', valueType: ValueType.Seconds },
    { key: 'glitchless_times', buttonLabel: 'Glitchless', tableTitle: 'Overall Glitchless Times', valueType: ValueType.Seconds  },
    { key: 'hundo_times', buttonLabel: '100%', tableTitle: 'Overall 100% Times', valueType: ValueType.Seconds  },
    { key: 'asylum_mastery', buttonLabel: 'Asylum', tableTitle: 'Asylum Mastery', valueType: ValueType.Percent },
    { key: 'city_mastery', buttonLabel: 'City', tableTitle: 'City Mastery', valueType: ValueType.Percent  },
    { key: 'origins_mastery', buttonLabel: 'Origins', tableTitle: 'Origins Mastery', valueType: ValueType.Percent  },
    { key: 'knight_mastery', buttonLabel: 'Knight', tableTitle: 'Knight Mastery', valueType: ValueType.Percent  },
    { key: 'overall_mastery', buttonLabel: 'Overall', tableTitle: 'Overall Mastery', valueType: ValueType.Percent  },
    { key: 'world_records', buttonLabel: 'WRs', tableTitle: '# of World Records', valueType: ValueType.Count  },
    { key: 'runner_times', buttonLabel: 'Times', tableTitle: 'Overall Leaderboard Times', valueType: ValueType.Seconds  },
    { key: 'runner_ranks', buttonLabel: 'Ranks', tableTitle: 'Overall Leaderboard Ranks', valueType: ValueType.Count  },
  ];

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

      {/* Buttons */}
      <div style={{ display: 'flex', flexWrap: 'wrap', justifyContent: 'center', gap: '10px', margin: '20px 0' }}>
        {datasetConfigs.map((btn) => (
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
        title={datasetConfigs.find((b) => b.key === selectedKey)?.tableTitle || 'Data'}
        tableKey="Runner"
        tableValue="Time"
        valueType={datasetConfigs.find((b) => b.key === selectedKey)?.valueType}
      />
    </div>
  );
}
