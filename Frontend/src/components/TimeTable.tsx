import React, { useState } from 'react';
import ExplainerModal from './ExplainerModal';

export enum ValueType{
  Seconds = "seconds",
  Count = "count",
  Percent = "percent"
}

type DataPair = {
  rank: number;
  player: string;
  value: number;
};

interface TimeTableProps{
	data: DataPair[];
	title: string;
	tableKey: string;
	tableValue: string;
	valueType: ValueType | undefined;
	explanation: string | undefined;
}

function formatSeconds(seconds: number): string{
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  const pad = (n: number) => n.toString().padStart(2, '0');
  return `${pad(hrs)}:${pad(mins)}:${pad(secs)}`;
}

const TimeTable: React.FC<TimeTableProps> = ({ data, title, tableKey, tableValue, valueType, explanation }) => {
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
            <th style={{ paddingLeft: '10px', paddingRight: '10px' }}>Rank</th>
            <th>{tableKey}</th>
            <th style={{ paddingLeft: '10px', paddingRight: '10px' }}>{tableValue}</th>
          </tr>
        </thead>
        <tbody>
          {
            data.map(({ rank, player, value }, index) => (
              <tr key={index}>
                <td style={{ textAlign: 'center' }}>{rank}</td>
                <td style={{ textAlign: 'left', padding: '5px' }}>{player}</td>
                <td style={{ textAlign: 'center', padding: '5px' }}>{formatValue(value)}</td>
              </tr>
            ))
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
