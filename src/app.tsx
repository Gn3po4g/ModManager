import 'virtual:uno.css';
import { invoke } from '@tauri-apps/api/core';
import { Transfer, type TransferProps } from 'antd';
import { useEffect, useState, useTransition } from 'react';

document.addEventListener('contextmenu', (e) => {
	e.preventDefault();
});

function App() {
	const [isPending, startTransition] = useTransition();
	const [allMods, setAllMods] = useState<string[]>([]);
	const [targetKeys, setTargetKeys] = useState<string[]>([]);
	const [selectedKeys, setSelectedKeys] = useState<string[]>([]);

	useEffect(() => {
		Promise.all([
			(invoke('get_all_mods') as Promise<string[]>).then(setAllMods),
			(invoke('get_enabled_mods') as Promise<string[]>).then(setTargetKeys),
		]).then(() => invoke('show_window'));
	}, []);

	const onChange: TransferProps['onChange'] = async (target, direction, moveKeys) => {
		const moveMods = allMods.filter((mod) => moveKeys.includes(mod)).sort();

		startTransition(async () => {
			await invoke(direction === 'left' ? 'disable_mods' : 'enable_mods', { mods: moveMods })
				.then(() => setTargetKeys(target as string[]))
				.catch(console.log);
		});
	};

	return (
		<div className='flex justify-center select-none'>
			<Transfer
				showSearch
				listStyle={{
					width: 265,
					height: 375,
				}}
				locale={{
					titles: ['可用mod', '已启用mod'],
					notFoundContent: null,
					searchPlaceholder: '搜索mod',
					itemUnit: '项',
					selectAll: '选择所有',
					deselectAll: '取消所有',
					selectInvert: '反选所有',
				}}
				dataSource={allMods.map((m) => ({ key: m }))}
				targetKeys={targetKeys?.sort()}
				selectedKeys={selectedKeys?.sort()}
				onChange={onChange}
				disabled={isPending}
				onSelectChange={(source, target) => setSelectedKeys([...source, ...target])}
				render={(mod) => mod.key}
			/>
		</div>
	);
}

export default App;
