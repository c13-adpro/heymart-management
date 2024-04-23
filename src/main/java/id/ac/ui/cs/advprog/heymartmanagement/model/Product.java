package id.ac.ui.cs.advprog.heymartmanagement.model;

import lombok.Builder;
import lombok.Getter;

@Builder
@Getter
public class Product {
    String productId;
    String productName;
    long productPrice;
    int productQuantity;

    public Product(String productId, String productName, long productPrice, int productQuantity) {
        this.productId = productId;
        this.productName = productName;
        this.productPrice = productPrice;
        this.productQuantity = productQuantity;
    }


}