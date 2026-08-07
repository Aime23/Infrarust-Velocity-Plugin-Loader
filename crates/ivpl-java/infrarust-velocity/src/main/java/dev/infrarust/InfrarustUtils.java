package dev.infrarust;

import net.kyori.adventure.text.Component;
import net.kyori.adventure.text.serializer.json.JSONComponentSerializer;

public class InfrarustUtils {
    public static String serializeComponent(Component component) {
        return JSONComponentSerializer.json().serialize(component);
    }

    public static Component deserializeComponent(String json) {
        return JSONComponentSerializer.json().deserialize(json);
    }
}
