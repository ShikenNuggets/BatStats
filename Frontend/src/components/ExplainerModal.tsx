import React from 'react';

interface ExplainerModalProps{
	title: string;
	explanation: string;
	onClose: () => void;
}

const ExplainerModal : React.FC<ExplainerModalProps> = ({ title, explanation, onClose }) => {
	return (
		<>
		<div className='modal-overlay' onClick={onClose}>
			<div className='modal-content' onClick={(e) => e.stopPropagation()}>
				<button className='modal-close-button' onClick={onClose} aria-label='Close modal'>X</button>
				<h1 style={{ paddingBottom: '10px', paddingRight: '25px' }}>{title}</h1>
				<p>{explanation}</p>
			</div>
		</div>
		</>
	);
};

export default ExplainerModal;
