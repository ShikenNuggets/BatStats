import React, { useState } from 'react';
import ExplainerModal from './ExplainerModal';

export enum ValueType{
  Seconds = "seconds",
  Count = "count",
  Percent = "percent",
  Date = "date"
}

type DataPair = {
  rank: number;
  player: string;
  value: number;
};

type WorldRecordInfo = {
  category_name: string;
  player_name: string;
  date: Date | string;
};

type TableDataType = 'stats' | 'world_records';

interface TimeTableProps{
	data: DataPair[] | WorldRecordInfo[];
	title: string;
	tableKey?: string;
	tableValue?: string;
	valueType?: ValueType;
	explanation?: string;
	dataType?: TableDataType;
}

function formatSeconds(seconds: number): string{
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  const pad = (n: number) => n.toString().padStart(2, '0');
  return `${pad(hrs)}:${pad(mins)}:${pad(secs)}`;
}

const TimeTable: React.FC<TimeTableProps> = ({ data, title, tableKey, tableValue, valueType, explanation, dataType = 'stats' }) => {
	const [modalOpen, setModalOpen] = useState(false);
	const openModal = () => {
		setModalOpen(true);
	}
	
	const closeModal = () => {
		setModalOpen(false);
	}

  const formatValue = (value: number) => {
    if (isNaN(value)){
      console.log("Value was NaN");
    }
    
    if (valueType === undefined){
      return value;
    }

    switch(valueType){
      case ValueType.Seconds:
        return formatSeconds(value);
      case ValueType.Percent:
        return `${(value * 100).toFixed(2)}%`;
			case ValueType.Count:
			default:
				return value;
    }
  };

  const formatDate = (date: Date | string): string => {
    const dateObj = typeof date === 'string' ? new Date(date) : date;
    return dateObj.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  };

  const isWorldRecordData = (item: any): item is WorldRecordInfo => {
    return 'category_name' in item && 'player_name' in item && 'date' in item && typeof item.category_name === 'string' && typeof item.player_name === 'string';
  };

	return(
		<div style={{ paddingTop: '25px' }}>
			{title && <h2 style={{ textAlign: 'center', paddingBottom: '5px' }}>
				{title}
				{explanation && (
					<sup style={{ fontSize: '0.5em' }} title={explanation} onClick={openModal}>?</sup>
				)}
			</h2>
			}
			<table style={{ display: 'inline-block', tableLayout: 'fixed' }} border={1} cellPadding={8}>
		<colgroup>
			<col style={{ width: '10%' }} />
			<col style={{ width: '10%' }} />
			<col style={{ width: '10%' }} />
		</colgroup>
        <thead>
          <tr>
            {dataType === 'world_records' ? (
              <>
                <th style={{ paddingLeft: '10px', paddingRight: '10px' }}>Category</th>
                <th>Player</th>
                <th style={{ paddingLeft: '10px', paddingRight: '10px' }}>Date</th>
              </>
            ) : (
              <>
                <th style={{ paddingLeft: '10px', paddingRight: '10px' }}>Rank</th>
                <th>{tableKey}</th>
                <th style={{ paddingLeft: '10px', paddingRight: '10px' }}>{tableValue}</th>
              </>
            )}
          </tr>
        </thead>
        <tbody>
          {
            data && data.length > 0 ? (
              data.map((item, index) => {
                if (dataType === 'world_records') {
                  const wrItem = item as WorldRecordInfo;
                  return (
                    <tr key={index}>
                      <td style={{ textAlign: 'left', padding: '5px' }}>{wrItem.category_name}</td>
                      <td style={{ textAlign: 'left', padding: '5px' }}>{wrItem.player_name}</td>
                      <td style={{ textAlign: 'center', padding: '5px' }}>{formatDate(wrItem.date)}</td>
                    </tr>
                  );
                } else {
                  const dataPair = item as DataPair;
                  return (
                    <tr key={index}>
                      <td style={{ textAlign: 'center' }}>{dataPair.rank}</td>
                      <td style={{ textAlign: 'left', padding: '5px' }}>{dataPair.player}</td>
                      <td style={{ textAlign: 'center', padding: '5px' }}>{formatValue(dataPair.value)}</td>
                    </tr>
                  );
                }
              })
            ) : (
              <tr>
                <td colSpan={3} style={{ textAlign: 'center', padding: '10px', paddingLeft: '100px', paddingRight: '100px' }}>
                  No data available.
                </td>
              </tr>
            )
          }
        </tbody>
      </table>
      {modalOpen && explanation && (
        <ExplainerModal title={title} explanation={explanation} onClose={closeModal} />
      )}
		</div>
	)
}

export default TimeTable;
