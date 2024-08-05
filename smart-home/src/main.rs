/**
 * Добавил все в один файл, пока что не разобрался ак импортить файлы
 */

trait Device {
    fn get_description(&self) -> String;
}

trait ElectricalDevice {
    fn get_capacity(&self) -> f32;
}

trait Sensor<T> {
    fn output(&self) -> T;
}

struct TemperatureSensor {}

struct ElectricalSocket {
    /**
     * Хотел тут в качестве типа использовать трейт ElectricalDevice, но компилятор стал ругаться на
     * динамическое связывание. Видимо из-за того, что размер реализации неизвестен на момент компиляции.
     * Пока непонятно как починить.
     */
    connected_device: TemperatureSensor,
    is_power_on: bool,
}

impl TemperatureSensor {
    const CAPACITY: f32 = 100.0;
}

impl Device for TemperatureSensor {
    fn get_description(&self) -> String {
        String::from("Temperature sensor")
    }
}

impl ElectricalDevice for TemperatureSensor {
    fn get_capacity(&self) -> f32 {
        Self::CAPACITY
    }
}

impl Sensor<f32> for TemperatureSensor {
    fn output(&self) -> f32 {
        todo!();
    }
}

impl ElectricalSocket {
    fn toggle(&mut self, is_power_on: bool) {
        self.is_power_on = is_power_on;
    }

    fn get_power_consumption(&self) -> f32 {
        self.connected_device.get_capacity()
    }
}

impl Device for ElectricalSocket {
    fn get_description(&self) -> String {
        String::from("Electrical socket")
    }
}

fn main() {
    let sensor = TemperatureSensor {};
    let mut socket = ElectricalSocket {
        connected_device: sensor,
        is_power_on: false,
    };

    socket.toggle(true);
    socket.get_power_consumption();
}
